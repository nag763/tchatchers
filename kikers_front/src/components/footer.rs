use yew::{Component, Html, html, Context, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            {"End of app"}
            </div>
        }
    }
}
