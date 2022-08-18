use yew::{Component, Html, html, Context, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub link: String,
}

pub struct Navlink;

impl Component for Navlink {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <a href={ctx.props().clone().link} class="inline-block text-sm px-4 py-2 leading-none text-white" name={ctx.props().clone().label}>
            {ctx.props().clone().label}
            </a>
        }
    }
}
