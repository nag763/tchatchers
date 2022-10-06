// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::router::Route;
use gloo_net::http::Request;
use yew::{html, Component, Context, Html};
use yew_router::{history::History, scope_ext::RouterScopeExt};

pub enum Msg {
    AccessVerified(bool),
}

#[derive(Default)]
pub struct SecuredRoute {
    pub content: Html,
    pub verified: bool,
}

impl Component for SecuredRoute {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AccessVerified(verified) => {
                if verified {
                    self.verified = true;
                } else {
                    ctx.link().history().unwrap().push(Route::SignIn);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.verified {
            let req = Request::get("/api/validate").send();
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = req.await.unwrap();
                link.send_message(Msg::AccessVerified(resp.ok()));
            });
        }

        html! {
            <>
                {self.content.clone()}
            </>
        }
    }
}
