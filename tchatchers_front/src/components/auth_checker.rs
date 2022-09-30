use crate::services::auth_bus::EventBus;
use gloo_net::http::Request;
use yew::{html, Component, Context, Html, Properties};
use yew_agent::Dispatched;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {}

pub struct AuthChecker;

impl Component for AuthChecker {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let req = Request::get("/api/validate").send();
        wasm_bindgen_futures::spawn_local(async move {
            let resp = req.await.unwrap();
            EventBus::dispatcher().send(resp.ok());
        });
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <></>
        }
    }
}
