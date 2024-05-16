use std::rc::Rc;

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use crate::components::common::AppButton;
use crate::components::common::Form;
use crate::components::common::FormFile;
use crate::components::common::FormFreeSection;
use crate::components::common::FormInput;
use crate::components::common::FormSelect;
use crate::components::common::WaitingForResponse;
use crate::router::Route;
use crate::utils::client_context::ClientContext;
use crate::utils::keyed_list::KeyedList;
use crate::utils::requester::Requester;
use modal_service::ModalBus;
use modal_service::ModalBusContent;
use modal_service::ModalContent;
use tchatchers_core::api_response::ApiResponse;
use tchatchers_core::locale::Locale;
use tchatchers_core::user::PartialUser;
use tchatchers_core::user::UpdatableUser;
use tchatchers_core::validation_error_message::ValidationErrorMessage;
use toast_service::{Alert, ToastBus};
use validator::Validate;
use web_sys::FormData;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::use_context;
use yew::AttrValue;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};
use yew_agent::worker::use_worker_subscription;
use yew_agent::worker::UseWorkerSubscriptionHandle;
use yew_router::scope_ext::RouterScopeExt;

#[function_component(SettingsHOC)]
pub fn feed_hoc() -> Html {
    let client_context = use_context::<Rc<ClientContext>>().expect("Context defined at startup");
    let bridge = use_worker_subscription::<ModalBus>();
    let toaster = use_worker_subscription::<ToastBus>();

    html! { <Settings context={client_context} {bridge} {toaster} /> }
}

pub enum Msg {
    SubmitForm,
    ErrorFromServer(ApiResponse),
    LocalError(AttrValue),
    ProfileUpdated(PartialUser),
    ConfirmDeletion,
    DeletionConfirmed,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    context: Rc<ClientContext>,
    bridge: UseWorkerSubscriptionHandle<ModalBus>,
    toaster: UseWorkerSubscriptionHandle<ToastBus>,
}

pub struct Settings {
    name: NodeRef,
    locale_id: NodeRef,
    new_pfp: NodeRef,
    wait_for_api: bool,
    server_error: Option<AttrValue>,
    ok_msg: Option<AttrValue>,
    user_context: ClientContext,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let context_ref = ctx.props().context.as_ref();

        Self {
            name: NodeRef::default(),
            locale_id: NodeRef::default(),
            user_context: context_ref.clone(),
            wait_for_api: false,
            server_error: None,
            ok_msg: None,
            new_pfp: NodeRef::default(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.bridge.len() < ctx.props().bridge.len() {
            let Some(last_msg) = ctx.props().bridge.last().cloned() else {
                panic!("Unreachable");
            };
            let last_msg: ModalBusContent = (*last_msg).clone();
            if let ModalBusContent::Outcome(true) = last_msg {
                ctx.link().send_message(Msg::DeletionConfirmed);
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitForm => {
                self.wait_for_api = true;
                self.ok_msg = None;
                self.server_error = None;
                self.new_pfp.cast::<HtmlInputElement>().unwrap();
                if let (Some(name), Some(locale_id), Some(new_pfp)) = (
                    self.name.cast::<HtmlInputElement>(),
                    self.locale_id.cast::<HtmlInputElement>(),
                    self.new_pfp.cast::<HtmlInputElement>(),
                ) {
                    if name.check_validity() {
                        let Ok(locale_id) = locale_id.value().parse() else {
                            ctx.link().send_message(Msg::LocalError(
                                "The given locale isn't valid".into(),
                            ));
                            return true;
                        };
                        let payload = UpdatableUser {
                            id: self.user_context.user.as_ref().unwrap().id,
                            locale_id,
                            name: name.value(),
                        };
                        let form_data = FormData::new().unwrap();
                        let bytes = bincode::serialize(&payload).unwrap();
                        form_data
                            .append_with_str("payload", std::str::from_utf8(&bytes).unwrap())
                            .unwrap();
                        if let Some(files) = new_pfp.files() {
                            if let Some(file) = &files.get(0) {
                                form_data
                                    .append_with_blob_and_filename("file", file, &new_pfp.value())
                                    .unwrap();
                            }
                        }

                        if let Err(e) = payload.validate() {
                            let message: ValidationErrorMessage = e.into();
                            ctx.link()
                                .send_message(Msg::LocalError(message.to_string().into()));
                        } else {
                            let bearer = ctx.props().context.bearer.clone();
                            let mut req = Requester::put("/api/user");
                            req.bearer(bearer.clone()).multipart_body(form_data);
                            let link = ctx.link().clone();
                            self.wait_for_api = true;
                            wasm_bindgen_futures::spawn_local(async move {
                                let resp = req.send().await;
                                if resp.ok() {
                                    let mut req = Requester::get("/api/whoami");
                                    let resp = req.bearer(bearer).send().await;
                                    if resp.ok() {
                                        let user: PartialUser =
                                            bincode::deserialize(&resp.binary().await.unwrap())
                                                .unwrap();

                                        link.send_message(Msg::ProfileUpdated(user));
                                    } else {
                                        link.send_message(Msg::ErrorFromServer(
                                            bincode::deserialize(&resp.binary().await.unwrap())
                                                .unwrap(),
                                        ));
                                    }
                                } else {
                                    link.send_message(Msg::ErrorFromServer(
                                        bincode::deserialize(&resp.binary().await.unwrap())
                                            .unwrap(),
                                    ));
                                }
                            });
                        }
                    }
                }
                false
            }
            Msg::ErrorFromServer(resp) => {
                self.wait_for_api = false;
                self.ok_msg = None;
                let err = ctx.props().context.translation.get_or_default(
                    &resp.label,
                    &resp.text.unwrap_or("A server error has been met".into()),
                );
                self.server_error = Some(err.into());
                true
            }
            Msg::ProfileUpdated(app_context) => {
                self.wait_for_api = false;
                let locale_id = app_context.locale_id;
                self.user_context.user.set(Some(app_context));
                let translation = Locale::find_by_id(locale_id).unwrap().translation_map;
                ctx.props().toaster.send(Alert {
                    is_success: true,
                    label: "profile_updated".into(),
                    default: "Your profile has been updated with success".into(),
                });
                if let Some(new_pfp) = self.new_pfp.cast::<HtmlInputElement>() {
                    new_pfp.set_value("");
                }
                self.ok_msg = Some(
                    translation
                        .get_or_default(
                            "profile_updated",
                            "Your profile has been updated with success",
                        )
                        .into(),
                );

                true
            }
            Msg::ConfirmDeletion => {
                let translation = ctx.props().context.translation.as_ref();
                let mc : ModalContent = ModalContent {
                    title: translation.get_or_default("modal_delete_profile_title", "You are about to delete your account"),
                    msg: translation.get_or_default("modal_delete_content", "This action is not reversible, once your account is deleted, there is no way for you to get it back."),
                    decline_text: Some(translation.get_or_default("modal_delete_profile_no", "I changed, my mind, don't delete my account")),
                    accept_text: Some(translation.get_or_default("modal_delete_profile_yes", "Understood, farewell")),
                };
                ctx.props().bridge.send(ModalBusContent::PopModal(mc));
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
                        link.send_message(Msg::ErrorFromServer(
                            bincode::deserialize(&resp.binary().await.unwrap()).unwrap(),
                        ));
                    }
                });
                true
            }
            Msg::LocalError(s) => {
                self.wait_for_api = false;
                self.ok_msg = None;
                self.server_error = Some(s);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let translation = &ctx.props().context.translation;
        let user = &ctx.props().context.user.as_ref().cloned().unwrap();
        let delete_profile_callback = {
            let link = ctx.link().clone();
            Callback::from(move |_: ()| link.send_message(Msg::ConfirmDeletion))
        };
        html! {

            <Form label="settings" {translation} default="Settings" onsubmit={ctx.link().callback(|_| Msg::SubmitForm)} form_error={&self.server_error} form_ok={&self.ok_msg}>
                  <FormInput label={"your_login_field"} {translation} default={"Your login"} value={user.login.clone()} disabled=true />
                  <FormInput label={"your_name_field"} {translation} default={"Your name"} value={user.name.clone()} minlength="3" maxlength="16" attr_ref={&self.name} />
                  <FormSelect label={"your_locale_field"} default={"Your locale"} {translation} attr_ref={&self.locale_id} default_value={AttrValue::from(user.locale_id.to_string())} values={KeyedList::from(Locale::get_keyed_list())} />
                  <FormFile label={"your_pfp_field"} default={"Your profile picture"} {translation} attr_ref={&self.new_pfp} current_path={user.pfp.clone()} accept={"image/*"}/>
                  <FormFreeSection>
                    <div class="flex items-center">
                    <div class="w-1/3"></div>
                    <div class="flex flex-row w-2/3 justify-end space-x-3">
                        if self.wait_for_api {
                            <WaitingForResponse {translation} />
                        } else {
                            <AppButton label={"delete_profile"} default={"Delete profile"} {translation} is_modal_opener=true callback={delete_profile_callback}/>
                            <AppButton label={"update_profile"} default={"Update profile"} {translation} />
                        }
                    </div>
                    </div>
                </FormFreeSection>
                </Form >
        }
    }
}
