use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props;

pub struct Settings;

impl Component for Settings {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>
            {"Settings"}
            </p>
        }
    }
}
