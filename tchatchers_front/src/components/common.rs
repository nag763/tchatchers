// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::borrow::Borrow;
use std::rc::Rc;
use tchatchers_core::locale::TranslationMap;
use web_sys::MouseEvent;
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::html::ChildrenRenderer;
use yew::virtual_dom::VChild;
use yew::{
    classes, function_component, html, use_force_update, use_memo, AttrValue, Callback, Children,
    Html, NodeRef, Properties,
};
use yew_router::prelude::Link;

use crate::router::Route;
use crate::utils::client_context::ClientContext;
use crate::utils::keyed_list::KeyedList;

use super::modal::MODAL_OPENER_CLASS;

#[derive(Clone, PartialEq, Properties)]
pub struct WaitingForResponseProperties {
    pub translation: Rc<TranslationMap>,
}

#[function_component(WaitingForResponse)]
pub fn waiting_for_response(props: &WaitingForResponseProperties) -> Html {
    let translation = &props.translation;
    html! {
        <p class="flex justify-center dark:text-gray-200">
        <I18N label={"waiting_for_server"} default={"Waiting for server..."} {translation} />
          <svg class="inline ml-2 w-6 h-6 text-gray-200 dark:text-zinc-200 animate-spin dark:text-gray-600 fill-gray-800" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
        </p>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormButtonProperties {
    #[prop_or_default]
    pub callback: Option<Callback<()>>,
    #[prop_or_default]
    pub is_modal_opener: bool,
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
}

#[function_component(FormButton)]
pub fn form_button(props: &FormButtonProperties) -> Html {
    html! {
      <div class="flex items-center">
        <div class="w-2/3"></div>
        <div class="w-1/3">
          <AppButton ..props.clone() />
        </div>
      </div>
    }
}

#[function_component(AppButton)]
pub fn app_button(props: &FormButtonProperties) -> Html {
    let button_type: &str = match props.callback {
        Some(_) => "button",
        None => "submit",
    };
    let callback = props.callback.clone();
    let onclick = move |_: MouseEvent| {
        if let Some(callback) = callback.borrow() {
            callback.emit(());
        }
    };
    let translation = &props.translation;
    html! {
        <button class={classes!("common-button", props.is_modal_opener.then_some(MODAL_OPENER_CLASS))} type={button_type} {onclick}>
        <I18N label={&props.label} {translation} default={&props.default}/>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct I18nProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
}

#[function_component(I18N)]
pub fn i18n(props: &I18nProperties) -> Html {
    html! {
        if let Some(translated) = props.translation.get(props.label.as_str()) {
            <>{translated}</>
        } else {
            <>{&props.default}</>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FormFileProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
    #[prop_or_default]
    pub attr_ref: NodeRef,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub accept: Option<AttrValue>,
    #[prop_or_default]
    pub current_path: Option<String>,
}

#[function_component(FormFile)]
pub fn file_attacher(props: &FormFileProperties) -> Html {
    let translation = &props.translation;
    let trigger = use_force_update();
    let is_a_file_uploaded = {
        let attr_ref = props.attr_ref.cast::<HtmlInputElement>();
        use_memo(attr_ref, |attr_ref| {
            if let Some(attr_ref) = attr_ref {
                attr_ref.value().ne("")
            } else {
                false
            }
        })
    };

    let svg_path = match *is_a_file_uploaded {
        true => "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",    
        false => "M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13",
    };

    html! {
        <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
            <label class="common-form-label" for="inline-full-name">
            <I18N label={&props.label} default={&props.default} {translation}/>
            </label>
            </div>
            <div class="md:w-2/3 flex justify-center items-center space-x-4 mt-2 dark:text-gray-200">
                <span class="dark:text-gray-300">
                    if let Some(v) = &props.current_path {
                        <><img class="h-10 w-10 rounded-full" src={v.clone()} /></>
                    } else if *is_a_file_uploaded {
                        <I18N label={"new_pfp_ready"} default={"Your new profile picture is ready to be uploaded"} {translation}/>
                    } else {
                        <I18N label={"no_pfp"} default={"You don't have any profile picture so far"} {translation}/>
                    }
                </span>
                <label for="file-upload">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d={svg_path} />
                </svg>
                </label>
                <input id="file-upload" type="file" hidden=true disabled={props.disabled} ref={&props.attr_ref} onchange={move |_| {gloo_console::log!("On change"); trigger.force_update();}} accept={props.accept.clone().unwrap_or_default()} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormInputProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
    #[prop_or_default]
    pub attr_ref: NodeRef,
    #[prop_or_default]
    pub minlength: Option<AttrValue>,
    #[prop_or_default]
    pub maxlength: Option<AttrValue>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub autofocus: bool,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProperties) -> Html {
    let translation = &props.translation;
    html! {
        <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
                <label class="common-form-label">
                <I18N label={&props.label} {translation} default={&props.default}/>
            </label>
            </div>
                <div class="md:w-2/3">
                <input autofocus={props.autofocus} class="common-input" type={&props.input_type} required={props.required} minlength={&props.minlength} maxlength={&props.maxlength} ref={&props.attr_ref} disabled={props.disabled} value={&props.value} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormSelectProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
    #[prop_or_default]
    pub attr_ref: NodeRef,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    pub values: KeyedList,
}

#[function_component(FormSelect)]
pub fn form_select(props: &FormSelectProperties) -> Html {
    let translation = &props.translation;
    html! {
        <div class="md:flex md:items-center mb-6">
            <div class="md:w-1/3">
            <label class="common-form-label" for="inline-full-name">
            <I18N label={&props.label} {translation} default={&props.default}/>
            </label>
            </div>
            <div class="md:w-2/3">
            <select class="common-input" id="inline-full-name" type="text" required=true ref={&props.attr_ref} value={&props.default_value} >
                {for props.values.iter().map(|l| html! {<option value={&l.0} selected={if let Some(value) = &props.default_value {value == &l.0} else {false}} >{&l.1}</option>})}
            </select>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormCheckboxProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
    #[prop_or_default]
    pub attr_ref: NodeRef,
}

#[function_component(FormCheckbox)]
pub fn form_checkbox(props: &FormCheckboxProperties) -> Html {
    let translation = &props.translation;
    html! {
        <div class="md:flex md:items-center mb-6">
        <div class="md:w-1/3"/>
        <div class="md:w-2/3">
            <div class="flex  items-center mr-4 space-x-2">
                <input type="checkbox" class="w-4 h-4 accent-purple-600 bg-zinc-600 dark:bg-zinc-600 dark:accent-zinc-700" ref={&props.attr_ref} />
                <label class="text-gray-500 dark:text-gray-200 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-keep-me-signed-in">
                <I18N label={&props.label} {translation} default={&props.default}/>
                </label>
            </div>
        </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormFreeSectionProperties {
    pub children: Children,
}

#[function_component(FormFreeSection)]
pub fn form_free_section(props: &FormFreeSectionProperties) -> Html {
    html! {
        <>
        {for props.children.iter()}
        </>
    }
}

#[derive(Clone, PartialEq, derive_more::From)]
pub enum FormSection {
    Input(VChild<FormInput>),
    Button(VChild<FormButton>),
    Select(VChild<FormSelect>),
    Checkbox(VChild<FormCheckbox>),
    FreeSection(VChild<FormFreeSection>),
    File(VChild<FormFile>),
}

impl From<FormSection> for Html {
    fn from(val: FormSection) -> Self {
        match val {
            FormSection::Input(child) => child.into(),
            FormSection::Button(child) => child.into(),
            FormSection::Select(child) => child.into(),
            FormSection::Checkbox(child) => child.into(),
            FormSection::FreeSection(child) => child.into(),
            FormSection::File(child) => child.into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FormProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Rc<TranslationMap>,
    #[prop_or_default]
    pub onsubmit: Option<Callback<SubmitEvent>>,
    pub children: ChildrenRenderer<FormSection>,
    #[prop_or_default]
    pub form_error: Option<AttrValue>,
    #[prop_or_default]
    pub form_ok: Option<AttrValue>,
}

#[function_component(Form)]
pub fn form(props: &FormProperties) -> Html {
    let onsubmit = props.onsubmit.clone();
    let translation = &props.translation;
    html! {
        <div class="flex items-center justify-center h-full dark:bg-zinc-800">
            <form class="w-full max-w-sm lg:max-w-md xl:max-w-lg border-2 border-slate-300 dark:border-zinc-600 px-6 py-6  lg:py-14 rounded-md" {onsubmit} action="javascript:void(0);">
            <h2 class="text-xl mb-10 text-center text-gray-600 dark:text-gray-200 font-bold"><I18N label={&props.label} {translation} default={&props.default} />
            </h2>
            { for props.children.iter() }
            if let Some(error) = &props.form_error {
                <small class="flex mt-4 mb-2 items-center text-red-500">
                {error}
                </small>
            }
            if let Some(ok_msg) = &props.form_ok {
                <small class="flex mt-4 mb-2 items-center text-green-500">
                {ok_msg}
                </small>
            }
            </form>
        </div>
    }
}

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="flex flex-col items-center align-middle justify-center h-screen text-2xl sm:text-8xl text-center text-slate-600 dark:text-gray-200 dark:bg-zinc-800">
            <img src="/favicon.ico" class="w-1/3 animate-pulse"/>
        </div>
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex items-center justify-center h-full text-2xl sm:text-8xl text-center text-slate-600 dark:text-gray-200 dark:bg-zinc-800">
        {"404 ( ˘︹˘ )"}
            <br/>
        {"This page doesn't exist"}
        </div>
    }
}

#[function_component(VerificationSucceeded)]
pub fn verification_succeeded() -> Html {
    let client_context = yew::use_context::<Rc<ClientContext>>().unwrap();
    let navigator = yew_router::prelude::use_navigator().unwrap();

    let translation = &client_context.translation;
    {
        let client_context = client_context.clone();
        wasm_bindgen_futures::spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(5_000).await;
            if (*client_context.user).is_some() {
                navigator.replace(&Route::JoinRoom);
            } else {
                navigator.replace(&Route::SignIn)
            }
        });
    }

    html! {
        <div class="flex flex-col justify-evenly items-center content-evenly h-full  text-center text-slate-600 dark:text-gray-200 dark:bg-zinc-800 animate-fade-out-slow">
            <p class="text-2xl sm:text-4xl lg:text-8xl w-4/5">{"ヾ(•＾▽＾•)"}</p>
            <p class="text-xl sm:text-2xl lg:text-4xl w-4/5">
                <I18N {translation} label={"verification_success"} default={"You have been verified with success, you will be redirected within few seconds to the main page."}></I18N>
            </p>
        </div>
    }
}

#[function_component(VerificationFailed)]
pub fn verification_failed() -> Html {
    let translation = &yew::use_context::<Rc<ClientContext>>().unwrap().translation;
    html! {
        <div class="flex flex-col justify-evenly items-center content-evenly h-full  text-center text-slate-600 dark:text-gray-200 dark:bg-zinc-800">
            <p class="text-2xl sm:text-4xl lg:text-8xl w-4/5 fade-out">{"( ˘︹˘ )"}</p>

            <div class="text-xl sm:text-2xl lg:text-4xl w-4/5">
                <p><I18N {translation} label={"verification_failed"} default={"It seeems like the verification failed or the link you are using has already been used."}></I18N></p>
                <p class="hover:underline">
                    <Link<Route> to={Route::Contact}>
                        <I18N {translation} label={"reach_an_admin"} default={"If needed reach an admin by clicking on this."}></I18N>
                    </Link<Route>>
                </p>
            </div>
        </div>
    }
}
