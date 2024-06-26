// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use gloo_net::http::{Method, Request, RequestBuilder, Response};
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use yew::{UseStateHandle, UseStateSetter};

use toast_service::{Alert, ToastBus};
use yew_agent::Spawnable;

const UNAUTHORIZED: u16 = 401u16;
const TOO_MANY_REQUESTS: u16 = 429u16;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Requester {
    endpoint: Option<String>,
    method: Option<Method>,
    payload: Option<JsValue>,
    is_bincode: bool,
    is_multipart: bool,
    bearer_value: Option<String>,
    bearer_setter: Option<UseStateSetter<Option<String>>>,
}

impl Requester {
    pub fn get(endpoint: &str) -> Self {
        Self {
            method: Some(Method::GET),
            endpoint: Some(String::from(endpoint)),
            ..Self::default()
        }
    }

    pub fn post(endpoint: &str) -> Self {
        Self {
            method: Some(Method::POST),
            endpoint: Some(String::from(endpoint)),
            ..Self::default()
        }
    }

    pub fn put(endpoint: &str) -> Self {
        Self {
            method: Some(Method::PUT),
            endpoint: Some(String::from(endpoint)),
            ..Self::default()
        }
    }

    pub fn delete(endpoint: &str) -> Self {
        Self {
            method: Some(Method::DELETE),
            endpoint: Some(String::from(endpoint)),
            ..Self::default()
        }
    }

    pub fn patch(endpoint: &str) -> Self {
        Self {
            method: Some(Method::PATCH),
            endpoint: Some(String::from(endpoint)),
            ..Self::default()
        }
    }

    pub fn body(&mut self, body: Option<impl Into<JsValue>>) -> &mut Self {
        if let Some(val) = body {
            self.payload = Some(val.into());
        }
        self
    }

    pub fn bincode_body<U: serde::Serialize>(&mut self, body: U) -> &mut Self {
        let bytes = bincode::serialize(&body).unwrap();
        let array = Uint8Array::from(&bytes[..]);
        self.payload = Some(array.into());
        self.is_bincode = true;
        self
    }

    pub fn multipart_body<U: Into<JsValue>>(&mut self, body: U) -> &mut Self {
        self.payload = Some(body.into());
        self.is_multipart = true;
        self
    }

    pub fn bearer(&mut self, bearer: UseStateHandle<Option<String>>) -> &mut Self {
        self.bearer_setter = Some(bearer.setter());
        self.bearer_value = bearer.as_ref().cloned();
        self
    }

    pub fn bearer_value(&mut self, bearer: String) -> &mut Self {
        self.bearer_value = Some(bearer);
        self
    }

    pub fn bearer_setter(&mut self, bearer_setter: UseStateSetter<Option<String>>) -> &mut Self {
        self.bearer_setter = Some(bearer_setter);
        self
    }

    pub async fn send(&mut self) -> Response {
        if let (Some(method), Some(endpoint)) = (&self.method, &self.endpoint) {
            let mut builder = RequestBuilder::new(endpoint);
            builder = builder.method(method.clone());
            if let Some(bearer) = &self.bearer_value {
                builder = builder.header("Authorization", &format!("Bearer {bearer}"));
            }
            if self.is_bincode {
                builder = builder.header("Content-Type", "application/bincode");
            }
            let req: Request = if let Some(payload) = &self.payload {
                builder.body(payload).unwrap()
            } else {
                builder.build().unwrap()
            };

            let resp = req.send().await.unwrap();
            if resp.status() == UNAUTHORIZED && endpoint != "/api/authenticate" {
                let reauth = Box::pin(
                    Self {
                        endpoint: Some("/api/authenticate".into()),
                        method: Some(Method::PATCH),
                        ..Self::default()
                    }
                    .send(),
                )
                .await;
                if reauth.ok() {
                    let new_token = reauth.text().await.unwrap();
                    if let Some(bearer_setter) = &self.bearer_setter {
                        bearer_setter.set(Some(new_token.clone()));
                    }
                    self.bearer_value = Some(new_token);
                    Box::pin(self.send()).await
                } else {
                    resp
                }
            } else {
                if resp.status() == TOO_MANY_REQUESTS {
                    ToastBus::spawner().spawn("./toast_spawn.js").send(Alert {
                        is_success: true,
                        label: "max_conns_reached".into(),
                        default: "The number of maxium simulatenous connections has been reached"
                            .into(),
                    });
                }
                resp
            }
        } else {
            panic!("You need to define both a endpoint and a method prior any call");
        }
    }
}
