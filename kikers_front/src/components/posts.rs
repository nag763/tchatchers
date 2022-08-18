use yew::{Component, Html, html, Context, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

pub struct Posts;

impl Component for Posts {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row-span-9">
            {"The app!"}
            </div>
        }
    }
}
