// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use reqwest_wasm::{Client, Method, Response};

#[derive(Default, Debug, Clone)]
pub struct Requester<T>
where
    T: serde::Serialize,
{
    endpoint: Option<String>,
    method: Option<Method>,
    payload: Option<T>,
    is_json: bool,
}

impl<T> Requester<T>
where
    T: serde::Serialize + std::default::Default + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn body(&mut self, body: Option<T>) -> &mut Self {
        self.payload = body;
        self
    }

    pub fn method(&mut self, method: Option<Method>) -> &mut Self {
        self.method = method;
        self
    }

    pub fn endpoint(&mut self, endpoint: Option<String>) -> &mut Self {
        self.endpoint = endpoint;
        self
    }

    pub fn is_json(&mut self, is_json: bool) -> &mut Self {
        self.is_json = is_json;
        self
    }

    pub async fn send(&self) -> Response {
        if let (Some(method), Some(endpoint)) = (self.method.clone(), &self.endpoint) {
            let client = Client::new();
            let location = web_sys::window().unwrap().location();
            let protocol = location.protocol().unwrap();
            let host = location.host().unwrap();
            let endpoint = format!("{}{}{}", protocol, host, endpoint);
            let builder = client.request(method, &endpoint);
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
            builder.send().await.unwrap()
        } else {
            panic!("You need to define both a endpoint and a method prior any call");
        }
    }
}
