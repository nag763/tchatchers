use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{Form, FormButton, FormFreeSection, FormInput, WaitingForResponse};
use crate::router::Route;
use crate::utils::client_context::ClientContext;
use crate::utils::requester::Requester;
use tchatchers_core::api_response::ApiResponse;
use tchatchers_core::locale::Locale;
use tchatchers_core::user::InsertableUser;
use tchatchers_core::validation_error_message::ValidationErrorMessage;
use toast_service::{Alert, ToastBus};
use validator::Validate;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, NodeRef, Properties,
};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;
use yew_router::scope_ext::RouterScopeExt;

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
    user_email: NodeRef,
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
                if let (
                    Some(login),
                    Some(name),
                    Some(password),
                    Some(password_confirmation),
                    Some(email),
                ) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.name.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                    self.password_confirmation.cast::<HtmlInputElement>(),
                    self.user_email.cast::<HtmlInputElement>(),
                ) {
                    let inputs = [&login, &name, &password];
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
                        let email = if !email.value().is_empty() {
                            Some(email.value())
                        } else {
                            None
                        };
                        let payload = InsertableUser {
                            login: login.value(),
                            name: name.value(),
                            password: password.value(),
                            email,
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
                                    let resp: ApiResponse =
                                        postcard::from_bytes(&resp.binary().await.unwrap())
                                            .unwrap();
                                    ToastBus::dispatcher().send(Alert {
                                        is_success: true,
                                        label: resp.label,
                                        default: if let Some(text) = resp.text {
                                            text
                                        } else {
                                            "User created with success".into()
                                        },
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(login) = self.login.cast::<HtmlInputElement>() {
                let _ = login.focus();
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = &ctx.props().client_context.translation;
        html! {
            <Form label="sign_up" {translation} default="Sign up" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} form_error={&self.server_error}>
            <FormInput label={"login"} autofocus=true {translation} default={"Login"} minlength="3" maxlength="32" attr_ref={&self.login} required=true />
            <FormInput label={"name_field"} {translation} default={"Name"} minlength="3" maxlength="16" attr_ref={&self.name} required=true />
            <FormInput label={"password_field"} {translation} default={"Password"} input_type="password" minlength="8" maxlength="128" attr_ref={&self.password} required=true />
            <FormInput label={"confirm_password"} {translation} default={"Confirm your password"} input_type="password" minlength="8" maxlength="128" attr_ref={&self.password_confirmation} required=true />
            <FormInput label={"your_email"} {translation} default={"Your email"} input_type="email" attr_ref={&self.user_email} required=false />

            <FormFreeSection>
                if self.wait_for_api {
                    <WaitingForResponse {translation} />
                } else {
                    <FormButton label={"sign_up"} default={"Sign up"} {translation} />
                }
            </FormFreeSection>
            </Form>
        }
    }
}
