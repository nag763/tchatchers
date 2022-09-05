use crate::router::Route;
use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use tchatchers_core::user::InsertableUser;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlButtonElement, HtmlInputElement};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};
use yew_router::history::History;
use yew_router::prelude::use_history;

const CHECK_LOGIN_AFTER: u32 = 1_500;

pub enum Msg {
    SubmitForm,
    OnInput,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct SignUp {
    login: NodeRef,
    password: NodeRef,
    name: NodeRef,
    button: NodeRef,
    former_login_value: String,
    check_login: Option<Timeout>,
}

impl Component for SignUp {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                if let (Some(login), Some(name), Some(password)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.name.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                ) {
                    let inputs = vec![&login, &name, &password];
                    if inputs.iter().all(|i| i.check_validity()) {
                        let req = Request::post("/api/create_user")
                            .header("content-type", "application/json")
                            .body(Some(&wasm_bindgen::JsValue::from_str(
                                &serde_json::to_string(&InsertableUser {
                                    login: login.value(),
                                    name: name.value(),
                                    password: password.value(),
                                })
                                .unwrap(),
                            )))
                            .send();
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.await.unwrap();
                            if resp.ok() {
                                let history = use_history().unwrap();
                                history.push(Route::SignIn);
                            } else {
                                gloo_console::log!(resp.text().await.unwrap());
                            }
                        });
                    }
                }
                true
            }
            Msg::OnInput => {
                if let (Some(login), Some(name), Some(password), Some(button)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.name.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                    self.button.cast::<HtmlButtonElement>(),
                ) {
                    let inputs = vec![&login, &name, &password];
                    button.set_disabled(inputs.iter().all(|i| i.check_validity()));
                    if self.former_login_value != login.value() {
                        self.former_login_value = login.value();
                        if login.min_length() <= login.value().len().try_into().unwrap() {
                            self.check_login = Some({
                                Timeout::new(CHECK_LOGIN_AFTER, move || {
                                    let button = button.clone();
                                    let login = login.clone();
                                    wasm_bindgen_futures::spawn_local(async move {
                                        let req = Request::get(&format!(
                                            "/api/login_exists/{}",
                                            login.value()
                                        ))
                                        .header("content-type", "application/json")
                                        .send();
                                        let resp = req.await.unwrap();
                                        if !resp.ok() {
                                            button.set_disabled(true);
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
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="flex items-center justify-center h-screen">
                <form class="w-full max-w-sm border-2 px-6 py-6" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Login"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" ref={&self.login} oninput={ctx.link().callback(|_| Msg::OnInput)}/>
                      <small class="mt-2 text-pink-600 text-sm" hidden=true>
                      {"This login is already taken by another user."}
                      </small>
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      {"Name"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500 visited:invalid:border-red-500 focus:invalid:border-red-500 invalid:visited:border-red-500" id="inline-full-name" type="text" required=true minlength="2" ref={&self.name} oninput={ctx.link().callback(|_| Msg::OnInput)} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      {"Password"}
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="4" ref={&self.password} oninput={ctx.link().callback(|_| Msg::OnInput)} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center">
                    <div class="md:w-1/3"></div>
                    <div class="md:w-2/3">
                      <button class="shadow bg-purple-500 enabled:hover:bg-purple-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded" type="submit" ref={&self.button}>
                      {"Sign Up"}
                      </button>
                    </div>
                  </div>
                </form>
                </div>
            </>
        }
    }
}
