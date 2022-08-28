use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef, Properties, Callback};
use gloo_timers::callback::{Timeout, Interval};

const PROGRESS_REFRESH: u32 = 20;
const TIMEOUT: u32 = 15_000;

pub enum Msg {
    FileAttached,
    IgnoreEvent,
    SubmitForm,
    ReactivateFields,
    UpdateProgress
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub pass_message_to_ws: Callback<String>

}

#[derive(Default)]
pub struct TypeBar {
    file: Option<String>,
    can_post: bool,
    attach_ref: NodeRef,
    input_ref: NodeRef,
    progress_percentage: u32,
    cooldown: Option<Timeout>,
    progress: Option<Interval>,

}

impl Component for TypeBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            can_post: true,
            ..Self::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileAttached => {
                if let Some(input) = self.attach_ref.cast::<HtmlInputElement>() {
                    self.file = Some(input.value());
                    true
                } else {
                    false
                }
            }
            Msg::SubmitForm => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    if !input.check_validity() {
                        return false;
                    }
                    ctx.props().pass_message_to_ws.emit(input.value());
                    input.set_value("");
                }
                if let Some(input) = self.attach_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                self.cooldown = Some({
                    let link = ctx.link().clone();
                    Timeout::new(TIMEOUT, move || link.send_message(Msg::ReactivateFields))
                });
                self.progress = Some({
                    let link = ctx.link().clone();
                    Interval::new(PROGRESS_REFRESH, move || {
                        link.send_message(Msg::UpdateProgress);
                    })
                });
                
                self.file = None;
                self.can_post = false;
                self.progress_percentage = 0;
                true
            },
            Msg::UpdateProgress => {
                self.progress_percentage += PROGRESS_REFRESH;
                true
            },
            Msg::ReactivateFields => {
                self.can_post = true;
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let svg_path = match &self.file {
            Some(_) => "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
            None => "M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13",
        };

        let placeholder_input = match self.can_post {
            true => "Type a message here",
            false => "Wait few seconds before typing...",
        };
        html! {
            <>
                <div class="grid justify-items-center content-center">
                    <label for="file-upload">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d={svg_path} />
                        </svg>
                    </label>
                    <input id="file-upload" type="file" ref={self.attach_ref.clone()} style="display: none;" disabled={!self.can_post} oninput={ctx.link().callback(|_| Msg::FileAttached)}/>
                </div>
                <div class="col-span-4">
                      <input class="shadow appearance-none border rounded py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-indigo-500 w-full h-10 invalid:border-red-500 disabled:bg-gray-100 focus:invalid:border-red-500" id="username" type="text" placeholder={placeholder_input} minlength="2" maxlength="127" ref={self.input_ref.clone()} disabled={!self.can_post} onkeydown={ctx.link().callback(|e : yew::KeyboardEvent | { if e.code() == "Enter" { Msg::SubmitForm } else { Msg::IgnoreEvent }})}/>
                      <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700" hidden={self.can_post}>
                        <div class="bg-gradient-to-r from-indigo-300 to-indigo-600 h-2.5 rounded-full" style={format!("width: {}%", self.progress_percentage*100/TIMEOUT)}>
                        </div>
                    </div>
                </div>
            <div class="flex justify-center">
              <button class="bg-indigo-500 enabled:hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-full h-10 border-solid border-2 border-indigo-500 h-10" onclick={ctx.link().callback(|_| Msg::SubmitForm)} disabled={!self.can_post} >

                  {"Post"}
              </button>
          </div>
        </>
        }
    }
}
