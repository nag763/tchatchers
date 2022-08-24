use super::filter::Filter;
use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct Filters;

impl Component for Filters {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="py-4 px-20 grid grid-cols-3 gap-4">
                <Filter name="All" />
                <Filter name="Male" />
                <Filter name="Female" />
            </div>
        }
    }
}
