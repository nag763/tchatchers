use super::postbar::Postbar;
use super::posts::Posts;
use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Feed;

impl Component for Feed {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Postbar />
                <Posts />
            </>
        }
    }
}
