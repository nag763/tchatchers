// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use yew::{html, Component, Context, Html};

pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="flex items-center justify-center h-full text-8xl text-center text-slate-600 dark:text-gray-200 dark:bg-zinc-800">
            {"404 ( ˘︹˘ )"}
                <br/>
            {"This route doesn't exist"}
            </div>
        }
    }
}
