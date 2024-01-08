use yew::{function_component, html, suspense::use_future, AttrValue, HtmlResult, Properties};

use crate::{router::Route, utils::requester::Requester};

#[derive(Clone, PartialEq, Properties)]
pub struct VerifyProperties {
    pub token: AttrValue,
}

#[function_component(Verify)]
pub fn verify(props: &VerifyProperties) -> HtmlResult {
    let token = &props.token;
    let mut req = Requester::post("/api/verify");
    req.postcard_body(token.as_bytes());
    let result = use_future(|| async move {
        let resp = req.send().await;
        if !resp.ok() {
            if let Ok(text) = resp.text().await {
                if let Ok(unserialized_text) = postcard::from_bytes::<&str>(text.as_bytes()) {
                    gloo_console::error!("Error : ", unserialized_text);
                }
            }
        }
        resp
    });
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let route = match result {
        Ok(v) if v.ok() => Route::VerificationSucceeded,
        _ => Route::VerificationFailed,
    };
    navigator.replace(&route);
    Ok(html! { <></> })
}
