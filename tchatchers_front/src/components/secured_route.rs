use crate::router::Route;
use gloo_net::http::Request;
use yew::{html, Component, Context, Html, Properties};
use yew_router::{history::History, scope_ext::RouterScopeExt};

pub enum Msg {
    AccessVerified(bool),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub component: Html,
}

#[derive(Default)]
pub struct SecuredRoute {
    pub content: Html,
    pub verified: bool,
}

impl Component for SecuredRoute {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AccessVerified(verified) => {
                if verified {
                    self.content = ctx.props().component.clone();
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
