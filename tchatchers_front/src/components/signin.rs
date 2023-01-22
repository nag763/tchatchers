// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{FormButton, WaitingForResponse};
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use crate::utils::requester::Requester;
use tchatchers_core::app_context::AppContext;
use tchatchers_core::user::AuthenticableUser;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, NodeRef,
    Properties, UseStateHandle,
};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;
use yew_router::scope_ext::RouterScopeExt;

use super::common::I18N;

#[function_component(SignInHOC)]
pub fn sign_in_hoc() -> Html {
    let app_context: UseStateHandle<Option<AppContext>> =
        use_context::<UseStateHandle<Option<AppContext>>>().expect("No app context");
    {
        let navigator = use_navigator().unwrap();
        if app_context.is_some() {
            navigator.replace(&Route::JoinRoom);
            ToastBus::dispatcher().send(Alert {
                is_success: false,
                content: "You are already logged in.".into(),
            });
        }
    }
    html! { <SignIn {app_context}/> }
}

pub enum Msg {
    SubmitForm,
    LoggedIn(AppContext),
    ErrorFromServer(AttrValue),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    app_context: UseStateHandle<Option<AppContext>>,
}

#[derive(Default)]
pub struct SignIn {
    login: NodeRef,
    password: NodeRef,
    server_error: Option<AttrValue>,
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
                        self.wait_for_api = true;
                        let payload = AuthenticableUser {
                            login: login.value(),
                            password: password.value(),
                        };
                        let mut req = Requester::<AuthenticableUser>::post("/api/authenticate");
                        req.is_json(true).body(Some(payload));
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.send().await;
                            if resp.status().is_success() {
                                let req = Requester::<()>::get("/api/app_context");
                                let resp = req.send().await;
                                if resp.status().is_success() {
                                    let app_context: AppContext =
                                        serde_json::from_str(&resp.text().await.unwrap()).unwrap();
                                    link.send_message(Msg::LoggedIn(app_context));
                                } else {
                                    link.send_message(Msg::ErrorFromServer(
                                        resp.text().await.unwrap().into(),
                                    ));
                                }
                            } else {
                                link.send_message(Msg::ErrorFromServer(
                                    resp.text().await.unwrap().into(),
                                ));
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
            Msg::LoggedIn(new_context) => {
                ctx.props().app_context.set(Some(new_context));
                ToastBus::dispatcher().send(Alert {
                    is_success: true,
                    content: "You logged in with success".into(),
                });
                ctx.link().navigator().unwrap().push(&Route::JoinRoom);
                false
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
                <div class="flex items-center justify-center h-full dark:bg-zinc-800">
                <form class="w-full max-w-sm border-2 dark:border-zinc-700 px-6 py-6  lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">

                <h2 class="text-xl mb-10 text-center text-gray-500 dark:text-gray-200 font-bold">{"Sign in"}</h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                        <I18N label={"login_field"} default={"Login"}/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" ref={&self.login} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      <I18N label={"password_field"} default={"Password"}/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="4" ref={&self.password} />
                    </div>
                  </div>
                  <small class="flex mt-4 mb-2 items-center text-red-500" hidden={self.server_error.is_none()}>
                    {self.server_error.as_ref().unwrap_or(&AttrValue::default())}
                  </small>
                    {end_of_form}
                </form>
                </div>
            </>
        }
    }
}
