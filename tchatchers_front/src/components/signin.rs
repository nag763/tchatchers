use crate::components::common::{FormButton, WaitingForResponse};
use crate::router::Route;
use gloo_net::http::Request;
use tchatchers_core::user::AuthenticableUser;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_router::{history::History, scope_ext::RouterScopeExt};
use crate::services::auth_bus::EventBus;
use yew_agent::Dispatched;

pub enum Msg {
    SubmitForm,
    ErrorFromServer(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct SignIn {
    login: NodeRef,
    password: NodeRef,
    server_error: Option<String>,
    wait_for_api: bool,
}

impl Component for SignIn {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                self.server_error = None;
                if let (Some(login), Some(password)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                ) {
                    let inputs = vec![&login, &password];
                    if inputs.iter().all(|i| i.check_validity()) {
                        let req = Request::post("/api/authenticate")
                            .header("content-type", "application/json")
                            .body(Some(&wasm_bindgen::JsValue::from_str(
                                &serde_json::to_string(&AuthenticableUser {
                                    login: login.value(),
                                    password: password.value(),
                                })
                                .unwrap(),
                            )))
                            .send();
                        let link = ctx.link().clone();
                        self.wait_for_api = true;
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.await.unwrap();
                            if resp.ok() {
                                EventBus::dispatcher().send(true);
                                link.history().unwrap().push(Route::Home);
                            } else {
                                link.send_message(Msg::ErrorFromServer(resp.text().await.unwrap()));
                            }
                        });
                    }
                }
                true
            }
            Msg::ErrorFromServer(s) => {
                self.server_error = Some(s);
                self.wait_for_api = false;
                if let Some(password) = self.password.cast::<HtmlInputElement>() {
                    password.set_value("");
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let end_of_form = match self.wait_for_api {
            true => html! { <WaitingForResponse /> },
            false => html! { <FormButton label="Sign in" /> },
        };
        html! {
            <>
                <div class="flex items-center justify-center h-full">
                <form class="w-full max-w-sm border-2 px-6 py-6  lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">

                <h2 class="text-xl mb-10 text-center text-gray-500 font-bold">{"Sign in"}</h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Login"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" ref={&self.login} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      {"Password"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="4" ref={&self.password} />
                    </div>
                  </div>
                  <small class="flex mt-4 mb-2 items-center text-red-500" hidden={self.server_error.is_none()}>
                    {self.server_error.as_ref().unwrap_or(&String::new())}
                  </small>
                    {end_of_form}
                </form>
                </div>
            </>
        }
    }
}
