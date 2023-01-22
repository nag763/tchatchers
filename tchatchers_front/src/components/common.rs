// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use js_sys::ArrayBuffer;
use tchatchers_core::translation::Translation;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, FileReader, InputEvent};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{function_component, html, use_state, AttrValue, Callback, Html, Properties};

use super::modal::MODAL_OPENER_CLASS;

#[function_component(WaitingForResponse)]
pub fn waiting_for_response() -> Html {
    html! {
        <p class="flex justify-center dark:text-gray-200">{"Waiting for server reply"}
          <svg class="inline ml-2 w-6 h-6 text-gray-200 dark:text-zinc-200 animate-spin dark:text-gray-600 fill-gray-800" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
        </p>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormButtonProperties {
    pub label: AttrValue,
    pub callback: Option<Callback<()>>,
    pub is_modal_opener: Option<bool>,
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
        if let Some(callback) = callback.clone() {
            callback.emit(());
        }
    };
    let additionnal_class: &str = match props.is_modal_opener {
        Some(v) if v => MODAL_OPENER_CLASS,
        _ => "",
    };
    html! {
        <button class={format!("shadow bg-zinc-800 dark:bg-gray-500 enabled:hover:bg-zinc-900 dark:enabled:hover:bg-gray-600 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded {}", additionnal_class)} type={button_type} {onclick}>
            {&props.label}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileAttacherProps {
    pub on_file_attached: Callback<Option<ArrayBuffer>>,
    pub disabled: bool,
    pub accept: Option<AttrValue>,
}

#[function_component(FileAttacher)]
pub fn file_attacher(props: &FileAttacherProps) -> Html {
    let is_file_attached = use_state(|| false);
    let svg_path = match *is_file_attached {
        true => "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
        false => "M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13",
    };

    let oninput_event = props.on_file_attached.clone();
    let onload = Closure::wrap(Box::new(move |event: Event| {
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let data = element.result().unwrap();
        let buffer: ArrayBuffer = data.dyn_into::<ArrayBuffer>().unwrap();
        is_file_attached.set(true);
        oninput_event.emit(Some(buffer));
    }) as Box<dyn FnMut(_)>);
    let fr = web_sys::FileReader::new().unwrap();

    let oninput = move |ie: InputEvent| {
        let target: Option<EventTarget> = ie.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let file = input.unwrap().files().unwrap();
        fr.set_onloadend(Some(onload.as_ref().unchecked_ref()));
        fr.read_as_array_buffer(&file.get(0).unwrap()).unwrap();
    };

    html! {
        <>
            <label for="file-upload">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d={svg_path} />
                </svg>
            </label>
            <input id="file-upload" type="file" hidden=true disabled={props.disabled} {oninput} accept={props.accept.clone().unwrap_or_default()} />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct I18nProperties {
    pub default: AttrValue,
    pub label: AttrValue,
    #[prop_or_default]
    pub translation: Option<Translation>,
}

#[function_component(I18N)]
pub fn i18n(props: &I18nProperties) -> Html {
    if let Some(translation) = &props.translation {
        if let Some(translated) = translation.get(props.label.as_str()) {
            return html! {<>{translated}</>};
        }
    }
    html! {<>{&props.default}</>}
}
