use gloo_timers::callback::{Interval, Timeout};
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};


#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[derive(Default)]
pub struct Postbar {
}

pub enum Msg {
}

impl Component for Postbar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            can_post: true,
            placeholder_input: String::from("Type a message"),
            ..Self::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                            <div class="grid grid-cols-6 py-5 px-5 gap-4 justify-center">
                            <div class="grid justify-items-center content-center">
                            <label hidden={self.file.is_some()} for="file-upload">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" />
                </svg>
                            </label>
                            <label hidden={self.file.is_none()} for="file-upload">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                </svg>
                            </label>
                                <input id="file-upload" type="file" ref={self.attach_ref.clone()} style="display: none;" oninput={ctx.link().callback(|_| Msg::FileAttached)}/>

                            </div>
                            <div class="col-span-4">
                              <input class="shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-indigo-500 w-full h-10 invalid:border-red-500 disabled:bg-gray-100" id="username" type="text" placeholder={self.placeholder_input.clone()} minlength="3" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post} oninput={ctx.link().callback(|_| Msg::InputChanged)} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::InputChanged }})} />
                          <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
          <div class="bg-gradient-to-r from-indigo-300 to-indigo-600 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}></div>
        </div>
                              </div>
                              <div class="flex justify-center">
                              <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::SubmitForm)} disabled={!self.can_post} >

                              {"Post"}
                              </button>
                              </div>
                              <div hidden={!self.error_on_input} >
                              <p class="text-red-500">{"Your message is either to short, or to long, ensure it matches the requirements"}</p>
                              </div>
                          </div>
                        }
    }
}
