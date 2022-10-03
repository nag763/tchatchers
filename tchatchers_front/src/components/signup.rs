use crate::components::common::{FormButton, WaitingForResponse};
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use crate::utils::requester::Requester;
use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use tchatchers_core::user::InsertableUser;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_agent::Dispatched;
use yew_router::prelude::History;
use yew_router::scope_ext::RouterScopeExt;

const CHECK_LOGIN_AFTER: u32 = 250;

pub enum Msg {
    SubmitForm,
    OnLoginChanged,
    ErrorFromServer(String),
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props;

#[derive(Default)]

pub struct SignUp {
    login: NodeRef,
    password: NodeRef,
    name: NodeRef,
    check_login: Option<Timeout>,
    wait_for_api: bool,
    server_error: Option<String>,
}

impl Component for SignUp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                self.server_error = None;
                if let (Some(login), Some(name), Some(password)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.name.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                ) {
                    let inputs = vec![&login, &name, &password];
                    if inputs.iter().all(|i| i.check_validity()) {
                        let link = ctx.link().clone();
                        self.wait_for_api = true;
                        let payload = InsertableUser {
                            login: login.value(),
                            name: name.value(),
                            password: password.value(),
                        };
                        let mut req = Requester::<InsertableUser>::post("/api/user");
                        req.is_json(true).body(Some(payload));
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.send().await;
                            if resp.status().is_success() {
                                ToastBus::dispatcher().send(Alert {
                                    is_success: true,
                                    content: "User created with success".into(),
                                });
                                link.history().unwrap().push(Route::SignIn);
                            } else {
                                link.send_message(Msg::ErrorFromServer(resp.text().await.unwrap()));
                            }
                        });
                    }
                }
                true
            }
            Msg::OnLoginChanged => {
                if let Some(login) = self.login.cast::<HtmlInputElement>() {
                    if login.min_length() <= login.value().len().try_into().unwrap() {
                        self.check_login = Some({
                            Timeout::new(CHECK_LOGIN_AFTER, move || {
                                wasm_bindgen_futures::spawn_local(async move {
                                    let req = Request::get(&format!(
                                        "/api/login_exists/{}",
                                        login.value()
                                    ))
                                    .header("content-type", "application/json")
                                    .send();
                                    let resp = req.await.unwrap();
                                    if !resp.ok() {
                                        login.set_custom_validity(
                                            "This login is already taken by another user.",
                                        );
                                    } else {
                                        login.set_custom_validity("");
                                    }
                                });
                            })
                        });
                    }
                }
                true
            }
            Msg::ErrorFromServer(e) => {
                self.wait_for_api = false;
                self.server_error = Some(e);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let end_of_form: Html = match self.wait_for_api {
            false => html! { <FormButton label="Sign up" /> },
            true => html! { <WaitingForResponse /> },
        };

        html! {
            <>
                <div class="flex items-center justify-center h-full dark:bg-zinc-800">
                <form class="w-full max-w-sm border-2 dark:border-zinc-700 px-6 py-6  lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">
                <h2 class="text-xl mb-10 text-center text-gray-500 dark:text-gray-200 font-bold">{"Sign up"}</h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Login"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" ref={&self.login} oninput={ctx.link().callback(|_| Msg::OnLoginChanged)}/>
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Name"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" type="text" required=true minlength="2" ref={&self.name} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      {"Password"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="4" ref={&self.password} />
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
