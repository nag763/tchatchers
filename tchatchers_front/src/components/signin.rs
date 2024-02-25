use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::{
    Form, FormButton, FormCheckbox, FormFreeSection, FormInput, WaitingForResponse,
};
use crate::router::Route;

use crate::utils::client_context::ClientContext;
use crate::utils::requester::Requester;
use tchatchers_core::api_response::{ApiGenericResponse, ApiResponse};
use tchatchers_core::user::{AuthenticableUser, PartialUser};
use toast_service::{Alert, ToastBus};
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_context, AttrValue, Component, Context, Html, NodeRef, Properties,
};
use yew_agent::Dispatched;
use yew_router::prelude::use_navigator;
use yew_router::scope_ext::RouterScopeExt;

#[function_component(SignInHOC)]
pub fn sign_in_hoc() -> Html {
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
    html! { <SignIn client_context={(*client_context).clone()}/> }
}

pub enum Msg {
    SubmitForm,
    LoggedIn(PartialUser),
    ErrorFromServer(ApiResponse),
    LocalError(AttrValue),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    client_context: ClientContext,
}

#[derive(Default)]
pub struct SignIn {
    login: NodeRef,
    password: NodeRef,
    remember_me: NodeRef,
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
                if let (Some(login), Some(password), Some(remember_me)) = (
                    self.login.cast::<HtmlInputElement>(),
                    self.password.cast::<HtmlInputElement>(),
                    self.remember_me.cast::<HtmlInputElement>(),
                ) {
                    let inputs = [&login, &password];
                    if inputs.iter().all(|i| i.check_validity()) {
                        self.wait_for_api = true;
                        let payload = AuthenticableUser {
                            login: login.value(),
                            password: password.value(),
                            session_only: !remember_me.checked(),
                        };
                        let mut req = Requester::post("/api/authenticate");
                        req.bincode_body(payload);
                        let link = ctx.link().clone();
                        let bearer = ctx.props().client_context.bearer.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let resp = req.send().await;
                            if resp.ok() {
                                let token = resp.text().await.unwrap();
                                bearer.set(Some(token));
                                let mut req = Requester::get("/api/whoami");
                                let resp = req.bearer(bearer).send().await;
                                if resp.ok() {
                                    let user: PartialUser =
                                        bincode::deserialize(&resp.binary().await.unwrap())
                                            .unwrap();
                                    link.send_message(Msg::LoggedIn(user));
                                } else {
                                    if resp.status() == 429u16 {
                                        link.send_message(Msg::ErrorFromServer(
                                            ApiGenericResponse::TooManyRequests.into(),
                                        ))
                                    }
                                    link.send_message(Msg::ErrorFromServer(
                                        bincode::deserialize(&resp.binary().await.unwrap())
                                            .unwrap(),
                                    ));
                                }
                            } else {
                                link.send_message(Msg::ErrorFromServer(
                                    bincode::deserialize(&resp.binary().await.unwrap()).unwrap(),
                                ));
                            }
                        });
                    }
                }
                true
            }
            Msg::ErrorFromServer(resp) => {
                let err = ctx.props().client_context.translation.get_or_default(
                    &resp.label,
                    &resp.text.unwrap_or("A server error has been met".into()),
                );
                self.server_error = Some(err.into());
                self.wait_for_api = false;
                if let Some(password) = self.password.cast::<HtmlInputElement>() {
                    password.focus().unwrap();
                    password.set_value("");
                }
                true
            }
            Msg::LoggedIn(new_context) => {
                ctx.props().client_context.user.set(Some(new_context));
                ToastBus::dispatcher().send(Alert {
                    is_success: true,
                    label: "login_success".into(),
                    default: "You logged in with success".into(),
                });
                ctx.link().navigator().unwrap().push(&Route::JoinRoom);
                false
            }
            Msg::LocalError(s) => {
                self.server_error = Some(s);
                self.wait_for_api = false;
                if let Some(password) = self.password.cast::<HtmlInputElement>() {
                    password.set_value("");
                }
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
            <Form label="sign_in" {translation} default="Sign in" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} form_error={&self.server_error}>
                <FormInput label={"login"} {translation} default={"Login"} minlength="3" attr_ref={&self.login} required=true autofocus=true />
                <FormInput label={"password_field"} {translation} required=true default={"Password"} minlength="4" input_type="password" attr_ref={&self.password} />
                <FormCheckbox label={"keep_me_signed_in"} {translation} default={"Remember me"} attr_ref={&self.remember_me}/>
                <FormFreeSection>
                    if self.wait_for_api {
                    <WaitingForResponse {translation} />
                    } else {
                    <FormButton label={"sign_in"} default={"Log in"} {translation} />
                    }
                </FormFreeSection>
            </Form>
        }
    }
}
