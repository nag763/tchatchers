use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::AppButton;
use crate::components::common::FileAttacher;
use crate::components::common::WaitingForResponse;
use crate::components::toast::Alert;
use crate::router::Route;
use crate::services::modal_bus::ModalBus;
use crate::services::modal_bus::ModalBusContent;
use crate::services::toast_bus::ToastBus;
use crate::utils::client_context::ClientContext;
use crate::utils::requester::Requester;
use tchatchers_core::app_context::UserContext;
use tchatchers_core::user::UpdatableUser;
use tchatchers_core::validation_error_message::ValidationErrorMessage;
use validator::Validate;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::use_context;
use yew::AttrValue;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};
use yew_agent::Bridge;
use yew_agent::Bridged;
use yew_agent::Dispatched;
use yew_router::scope_ext::RouterScopeExt;

use super::common::I18N;
use super::modal::ModalContent;

#[function_component(SettingsHOC)]
pub fn feed_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().expect("Context defined at startup");

    html! { <Settings context={client_context} /> }
}

pub enum Msg {
    UploadNewPfp(Option<js_sys::ArrayBuffer>),
    PfpUpdated(AttrValue),
    SubmitForm,
    ErrorFromServer(AttrValue),
    ProfileUpdated(UserContext),
    ConfirmDeletion,
    DeletionConfirmed,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    context: Rc<ClientContext>,
}

pub struct Settings {
    name: NodeRef,
    locale_id: NodeRef,
    pfp: Option<String>,
    wait_for_api: bool,
    server_error: Option<AttrValue>,
    ok_msg: Option<AttrValue>,
    producer: Box<dyn Bridge<ModalBus>>,
    user_context: UserContext,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |mc| {
                if let ModalBusContent::Outcome(true) = mc {
                    link.send_message(Msg::DeletionConfirmed)
                }
            }
        };

        let context_ref = ctx.props().context.user_context.as_ref().unwrap();

        Self {
            name: NodeRef::default(),
            locale_id: NodeRef::default(),
            pfp: context_ref.user.pfp.clone(),
            user_context: context_ref.clone(),
            wait_for_api: false,
            server_error: None,
            ok_msg: None,
            producer: ModalBus::bridge(Rc::new(cb)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                self.wait_for_api = true;
                self.ok_msg = None;
                self.server_error = None;
                if let (Some(name), Some(locale_id)) = (
                    self.name.cast::<HtmlInputElement>(),
                    self.locale_id.cast::<HtmlInputElement>(),
                ) {
                    if name.check_validity() {
                        let Ok(locale_id) = locale_id.value().parse() else {
                            ctx.link().send_message(Msg::ErrorFromServer("The given locale isn't valid".into()));
                            return true;
                        };
                        let payload = UpdatableUser {
                            id: self.user_context.user.id,
                            locale_id,
                            name: name.value(),
                            pfp: self.pfp.clone(),
                        };
                        if let Err(e) = payload.validate() {
                            let message: ValidationErrorMessage = e.into();
                            ctx.link()
                                .send_message(Msg::ErrorFromServer(message.to_string().into()));
                        } else {
                            let bearer = ctx.props().context.bearer.clone();
                            let mut req = Requester::put("/api/user");
                            req.is_json(true).bearer(bearer.clone()).json_body(payload);
                            let link = ctx.link().clone();
                            self.wait_for_api = true;
                            let translation = self.user_context.translation.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let resp = req.send().await;
                                if resp.ok() {
                                    let mut req = Requester::get("/api/app_context");
                                    let resp = req.bearer(bearer).send().await;
                                    if resp.ok() {
                                        let app_context: UserContext =
                                            serde_json::from_str(&resp.text().await.unwrap())
                                                .unwrap();
                                        ToastBus::dispatcher().send(Alert {
                                            is_success: true,
                                            content: translation.as_ref().get_or_default(
                                                "profile_updated",
                                                "Your profile has been updated with success",
                                            ),
                                        });
                                        link.send_message(Msg::ProfileUpdated(app_context));
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
                }
                true
            }
            Msg::UploadNewPfp(pfp) => {
                self.wait_for_api = true;
                let mut req = Requester::post("/api/pfp");
                req.body(pfp);
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = req.send().await;
                    if resp.ok() {
                        link.send_message(Msg::PfpUpdated(resp.text().await.unwrap().into()));
                    } else {
                        link.send_message(Msg::ErrorFromServer(resp.text().await.unwrap().into()));
                    }
                });
                true
            }
            Msg::ErrorFromServer(s) => {
                self.wait_for_api = false;
                self.ok_msg = None;
                self.server_error = Some(s);
                true
            }
            Msg::PfpUpdated(pfp_path) => {
                self.wait_for_api = false;
                self.pfp = Some(pfp_path.to_string());
                true
            }
            Msg::ProfileUpdated(app_context) => {
                self.wait_for_api = false;
                self.user_context = app_context.clone();
                ctx.props().context.user_context.set(Some(app_context));

                self.ok_msg = Some(
                    self.user_context
                        .translation
                        .clone()
                        .get_or_default(
                            "profile_updated",
                            "Your profile has been updated with success",
                        )
                        .into(),
                );

                true
            }
            Msg::ConfirmDeletion => {
                let translation = self.user_context.translation.as_ref();
                let mc : ModalContent = ModalContent {
                    title: translation.get_or_default("modal_delete_profile_title", "You are about to delete your account"),
                    msg: translation.get_or_default("modal_delet", "This action is not reversible, once your account is deleted, there is no way for you to get it back."),
                    decline_text: Some(translation.get_or_default("modal_delete_profile_no", "I changed, my mind, don't delete my account")),
                    accept_text: Some(translation.get_or_default("modal_delete_profile_yes", "Understood, farewell")),
                };
                self.producer.send(ModalBusContent::PopModal(mc));
                false
            }
            Msg::DeletionConfirmed => {
                let mut req = Requester::delete("/api/user");
                let link = ctx.link().clone();
                self.wait_for_api = true;
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = req.send().await;
                    if resp.ok() {
                        link.navigator().unwrap().push(&Route::LogOut);
                    } else {
                        link.send_message(Msg::ErrorFromServer(resp.text().await.unwrap().into()));
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = self.user_context.translation.as_ref();
        let pfp = match &self.user_context.user.pfp {
            None => match &self.pfp {
                Some(_) => {
                    html! {<span class="dark:text-gray-300">{"Your new profile picture is ready to be uploaded"}</span>}
                }
                None => {
                    html! { <span class="dark:text-gray-300">{"You don't have any profile picture so far"}</span> }
                }
            },
            Some(v) => html! { <><img class="h-10 w-10 rounded-full" src={v.clone()} /></> },
        };
        let link = ctx.link().clone();
        let end_of_form = match self.wait_for_api {
            true => html! { <WaitingForResponse /> },
            false => {
                html! { <AppButton label={translation.get_or_default("update_profile", "Update profile")} /> }
            }
        };
        let delete_profile = match self.wait_for_api {
            true => html! { <WaitingForResponse /> },
            false => {
                html! { <AppButton label={translation.get_or_default("delete_profile", "Delete profile")} is_modal_opener=true callback={Callback::from(move |_ :()| {link.send_message(Msg::ConfirmDeletion)})}/> }
            }
        };
        let link = ctx.link().clone();
        html! {
            <>
                <div class="flex items-center justify-center h-full dark:bg-zinc-800">
                <form class="w-full max-w-sm border-2 dark:border-zinc-700 px-6 py-6  lg:py-14" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} action="javascript:void(0);" >

                <h2 class="text-xl mb-10 text-center text-gray-500 dark:text-gray-200 font-bold">
                    <I18N label={"settings"} default={"Settings"} translation={self.user_context.translation.clone()}/>
                </h2>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                        <I18N label={"your_login_field"} default={"Your login"} translation={self.user_context.translation.clone()}/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" maxlength="32" value={self.user_context.user.login.clone()} disabled=true/>
                    </div>
                    </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      <I18N label={"your_name_field"} default={"Your name"} translation={self.user_context.translation.clone()}/>
                      </label>
                    </div>
                    <div class="md:w-2/3">
                      <input class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true minlength="3" maxlength="16" ref={&self.name} value={self.user_context.user.name.clone()}/>
                    </div>
                  </div>
                  <div class="md:flex md:items-center mb-6">
                  <div class="md:w-1/3">
                    <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                    <I18N label={"your_locale_field"} default={"Your locale"} translation={self.user_context.translation.clone()}/>
                    </label>
                  </div>
                  <div class="md:w-2/3">
                    <select class="peer bg-gray-200 dark:bg-zinc-800 appearance-none border-2 border-gray-200 dark:border-zinc-700 rounded w-full py-2 px-4 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:bg-white dark:focus:bg-zinc-800 focus:border-zinc-500 focus:invalid:border-red-500 visited:invalid:border-red-500" id="inline-full-name" type="text" required=true ref={&self.locale_id} >
                        {self.user_context.available_locale.iter().map(|l|
                                html! {<option value={l.id.to_string()} selected={l.id == self.user_context.user.locale_id}>{l.long_name.as_str()}</option>}
                        ).collect::<Html>()}
                    </select>
                  </div>
                </div>
                  <div class="md:flex md:items-center mb-6">
                    <div class="md:w-1/3">
                      <label class="block text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
                      <I18N label={"your_pfp_field"} default={"Your profile picture"} translation={self.user_context.translation.clone()}/>
                      </label>
                    </div>
                    <div class="md:w-2/3 flex justify-center items-center space-x-4 mt-2 dark:text-gray-200">
                    {pfp}
                    <FileAttacher disabled=false accept={Some(AttrValue::from(".png,.webp,.jpg,.jpg"))} on_file_attached={Callback::from(move |file_path: Option<js_sys::ArrayBuffer>| {
                        link.send_message(Msg::UploadNewPfp (file_path));
        })}/>
                    </div>
                  </div>
                  <small class="flex mt-4 mb-2 items-center text-red-500" hidden={self.server_error.is_none()}>
                    {self.server_error.as_ref().unwrap_or(&AttrValue::default())}
                  </small>
                  <small class="flex mt-4 mb-2 items-center text-green-500" hidden={self.ok_msg.is_none()}>
                    {self.ok_msg.as_ref().unwrap_or(&AttrValue::default())}
                  </small>
                  <div class="flex items-center">
                  <div class="w-1/3"></div>
                  <div class="flex flex-row w-2/3 justify-end space-x-3">
                     {delete_profile}
                     {end_of_form}
                  </div>
                </div>
                </form>
                </div>
            </>
        }
    }
}
