use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props;

pub struct SignIn;

impl Component for SignIn {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <input type="text" />
            <input type="text" />
            <input type="text" />
            <button>{"Sign in"}</button>
            </>
        }
    }
}
