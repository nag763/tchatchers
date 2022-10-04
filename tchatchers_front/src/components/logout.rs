use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::auth_bus::EventBus;
use crate::services::toast_bus::ToastBus;
use gloo_net::http::Request;
use yew::{html, Component, Context, Html, Properties};
use yew_agent::Dispatched;
use yew_router::history::History;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props;

pub struct LogOut;

impl Component for LogOut {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let req = Request::get("/api/logout").send();
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            req.await.unwrap();
            EventBus::dispatcher().send(false);
            ToastBus::dispatcher().send(Alert {
                is_success: true,
                content: "You logged out with success".into(),
            });
            link.history().unwrap().replace(Route::SignIn);
        });

        html! {
            <div class="dark:bg-zinc-800">
            </div>
        }
    }
}
