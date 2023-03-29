// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use async_recursion::async_recursion;
use reqwest_wasm::{Client, Method, Response, StatusCode};
use yew::{UseStateHandle, UseStateSetter};

#[derive(Default, Debug, Clone)]
pub struct Requester<T>
where
    T: serde::Serialize,
{
    endpoint: Option<String>,
    method: Option<Method>,
    payload: Option<T>,
    is_json: bool,
    bearer_value: Option<String>,
    bearer_setter: Option<UseStateSetter<Option<String>>>,
}

impl<T> Requester<T>
where
    T: serde::Serialize + std::default::Default + Clone,
{
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

    pub fn body(&mut self, body: Option<T>) -> &mut Self {
        self.payload = body;
        self
    }

    pub fn is_json(&mut self, is_json: bool) -> &mut Self {
        self.is_json = is_json;
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

    #[async_recursion(?Send)]
    pub async fn send(&mut self) -> Response {
        if let (Some(method), Some(endpoint)) = (&self.method, &self.endpoint) {
            let client = Client::new();
            let location = web_sys::window().unwrap().location();
            let protocol = location.protocol().unwrap();
            let host = location.host().unwrap();
            let uri = format!("{}{}{}", protocol, host, endpoint);
            let builder = client.request(method.clone(), &uri);
            let builder = match (self.payload.clone(), self.is_json) {
                (Some(payload), true) => {
                    let serde_struct: String = serde_json::to_string(&payload).unwrap();
                    builder
                        .body(serde_struct)
                        .header("content-type", "application/json")
                }
                (None, true) => builder.header("content-type", "application/json"),
                _ => builder,
            };

            let resp = match &self.bearer_value {
                Some(bearer) => builder.bearer_auth(bearer).send().await.unwrap(),
                _ => builder.send().await.unwrap(),
            };
            if resp.status() == StatusCode::UNAUTHORIZED && endpoint != "/api/authenticate" {
                let reauth = Self {
                    endpoint: Some("/api/authenticate".into()),
                    method: Some(Method::PATCH),
                    ..Self::default()
                }
                .send()
                .await;
                if reauth.status().is_success() {
                    let new_token = reauth.text().await.unwrap();
                    if let Some(bearer_setter) = &self.bearer_setter {
                        bearer_setter.set(Some(new_token.clone()));
                    }
                    self.bearer_value = Some(new_token);
                    self.send().await
                } else {
                    resp
                }
            } else {
                resp
            }
        } else {
            panic!("You need to define both a endpoint and a method prior any call");
        }
    }
}
