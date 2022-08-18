use yew::{Component, Html, html, Context, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Postbar;

impl Component for Postbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p>
            {"The postbar"}
            </p>
        }
    }
}
