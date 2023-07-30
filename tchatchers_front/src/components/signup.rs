use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{FormButton, WaitingForResponse, I18N};
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::toast_bus::ToastBus;
use crate::utils::client_context::ClientContext;
use crate::utils::requester::Requester;
use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use tchatchers_core::api_response::ApiResponse;
use tchatchers_core::locale::Locale;
use tchatchers_core::user::InsertableUser;
use tchatchers_core::validation_error_message::ValidationErrorMessage;
use validator::Validate;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, NodeRef, Properties,
};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;
use yew_router::scope_ext::RouterScopeExt;

const CHECK_LOGIN_AFTER: u32 = 250;

#[function_component(SignUpHOC)]
pub fn sign_up_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().expect("No app context");
    {
        let navigator = use_navigator().unwrap();
        if client_context.user.is_some() {
            navigator.replace(&Route::JoinRoom);
            ToastBus::dispatcher().send(Alert {
                is_success: false,
                label: "already_logged_in".into(),
                default: "You are already logged in".into(),
            });
        }
    }
    html! { <SignUp client_context={(*client_context).clone()}/> }
}

pub enum Msg {
    SubmitForm,
    OnLoginChanged,
    ErrorFromServer(ApiResponse),
    LocalError(AttrValue),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    client_context: ClientContext,
}

#[derive(Default)]

pub struct SignUp {
    login: NodeRef,
    password: NodeRef,
    name: NodeRef,
    password_confirmation: NodeRef,
    check_login: Option<Timeout>,
    wait_for_api: bool,
    server_error: Option<AttrValue>,
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
                if let (Some(login), Some(name), Some(password), Some(password_confirmation)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.name.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                    self.password_confirmation.cast::<HtmlInputElement>(),
                ) {
                    let inputs = vec![&login, &name, &password];
                    if inputs.iter().all(|i| i.check_validity()) {
                        let locale = ctx
                            .props()
                            .client_context
                            .locale
                            .as_ref()
                            .clone()
                            .unwrap_or(Locale::get_default_locale())
                            .id;
                        let link = ctx.link().clone();
                        self.wait_for_api = true;
                        let payload = InsertableUser {
                            login: login.value(),
                            name: name.value(),
                            password: password.value(),
                            locale,
                        };
                        if let Err(e) = payload.validate() {
                            let message: ValidationErrorMessage = e.into();
                            link.send_message(Msg::LocalError(message.to_string().into()));
                        } else if !password.value().eq(&password_confirmation.value()) {
                            password.set_value("");
                            password_confirmation.set_value("");
                            link.send_message(Msg::LocalError(
                                ctx.props()
                                    .client_context
                                    .translation
                                    .get_or_default(
                                        "passwords_dont_match",
                                        "The passwords do not match",
                                    )
                                    .into(),
                            ));
                        } else {
                            let mut req = Requester::post("/api/user");
                            req.postcard_body(payload);
                            wasm_bindgen_futures::spawn_local(async move {
                                let resp = req.send().await;
                                if resp.ok() {
                                    ToastBus::dispatcher().send(Alert {
                                        is_success: true,
                                        label: "success_on_user_creation".into(),
                                        default: "User created with success".into(),
                                    });
                                    link.navigator().unwrap().push(&Route::SignIn);
                                } else {
                                    link.send_message(Msg::ErrorFromServer(
                                        postcard::from_bytes(&resp.binary().await.unwrap())
                                            .unwrap(),
                                    ));
                                }
                            });
                        }
                    }
                }
                true
            }
            Msg::OnLoginChanged => {
                if let Some(login) = self.login.cast::<HtmlInputElement>() {
                    if login.min_length() <= login.value().len().try_into().unwrap() {
                        let translation = ctx.props().client_context.translation.clone();
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
                                        login.set_custom_validity(&translation.get_or_default(
                                            "login_already_taken",
                                            "The login is already token by another user",
                                        ));
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
            Msg::ErrorFromServer(resp) => {
                self.wait_for_api = false;
                let err = ctx.props().client_context.translation.get_or_default(
                    &resp.label,
                    &resp.text.unwrap_or("A server error has been met".into()),
                );
                self.server_error = Some(err.into());
                true
            }
            Msg::LocalError(e) => {
                self.wait_for_api = false;
                self.server_error = Some(e);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = ctx.props().client_context.translation.clone();
        let end_of_form: Html = match self.wait_for_api {
            false => {
                html! { <FormButton label={translation.get_or_default("sign_up", "Sign up")} /> }
            }
            true => html! { <WaitingForResponse translation={translation.clone()} /> },
        };

        html! {
            <>
                <div class="flex items-center justify-center h-full dark:bg-zinc-800">
                <form class="w-full max-w-sm border-2 dark:border-zinc-700 px-6 py-6  lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);">
                <h2 class="text-xl mb-10 text-center text-gray-500 dark:text-gray-200 font-bold"><I18N label="sign_up" translation={translation.clone()} default="Sign up"/></h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      <I18N label="login" translation={translation.clone()} default="Login"/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" maxlength="32" ref={&self.login} oninput={ctx.link().callback(|_| Msg::OnLoginChanged)}/>
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      <I18N label="name_field" translation={translation.clone()} default="Name"/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" type="text" required=true minlength="3" maxlength="16" ref={&self.name} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      <I18N label="password_field" translation={translation.clone()} default="Password"/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="8" maxlength="128" ref={&self.password} />
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-password">
                      <I18N label="confirm_password" translation={translation.clone()} default="Confirm your password"/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-password" type="password" required=true minlength="8" maxlength="128" ref={&self.password_confirmation} />
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
