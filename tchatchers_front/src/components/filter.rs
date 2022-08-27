use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct Filter;

impl Component for Filter {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button class="bg-white hover:bg-indigo-100 text-indigo-500 font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500">
                {ctx.props().clone().name}
            </button>
        }
    }
}
