use yew::{function_component, suspense::use_future, AttrValue, Html, HtmlResult, Properties};

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
    let result = use_future(|| async move { req.send().await })?;
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let route = if result.ok() {
        Route::VerificationSucceeded
    } else {
        Route::VerificationFailed
    };
    navigator.replace(&route);
    Ok(Html::default())
}
