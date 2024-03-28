// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use std::rc::Rc;

use tchatchers_core::locale::Locale;
use tchatchers_core::navlink::Navlink;
use tchatchers_core::user::PartialUser;
use tchatchers_front::components::prelude::*;

use chat_service::ChatReactor;
use modal_service::ModalBus;
use rmenu_service::RMenuBus;
use tchatchers_front::components::toast::ToastHOC;
use tchatchers_front::router::{switch, Route};
use tchatchers_front::utils::client_context::ClientContext;
use tchatchers_front::utils::requester::Requester;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_agent_latest::{reactor::ReactorProvider, worker::WorkerProvider, Bincode};
use yew_router::prelude::*;

#[function_component(ContextualApp)]
fn contextual_app() -> HtmlResult {
    let bearer: UseStateHandle<Option<String>> = use_state(|| None);

    let user = {
        let bearer_setter = bearer.setter();
        use_future(|| async {
            let mut req = Requester::get("/api/whoami");
            let resp = req.bearer_setter(bearer_setter).send().await;
            if resp.ok() {
                let user: PartialUser =
                    postcard::from_bytes(&resp.binary().await.unwrap()).unwrap();
                Some(user)
            } else {
                None
            }
        })?
    };

    let user = use_state(|| user.clone());

    let navigator_language: Option<Vec<String>> =
        tchatchers_front::utils::language::get_navigator_languages();

    let locale = use_memo((*user).clone(), |user| {
        if let Some(user) = user {
            Locale::find_by_id(user.locale_id)
        } else if let Some(navigator_language) = navigator_language {
            Locale::get_for_web_names(navigator_language)
        } else {
            None
        }
    });

    let translation = use_memo((*locale).clone(), |locale| {
        if let Some(locale) = locale {
            locale.clone().translation_map
        } else {
            Locale::get_default_locale().translation_map
        }
    });

    let navlink = use_memo((*user).clone(), |user| {
        if let Some(user) = user {
            Navlink::get_visibility_for_profile(Some(user.profile))
        } else {
            Navlink::get_visibility_for_profile(None)
        }
    });

    let context = Rc::new(ClientContext {
        user,
        bearer,
        available_locale: Locale::get_available_locales(),
        translation,
        navlink,
        locale,
    });

    Ok(html! {
        <BrowserRouter>
            <ReactorProvider<ChatReactor> path="/chat_service.js">
            <WorkerProvider<ModalBus, Bincode> path="/modal_service.js">
            <WorkerProvider<RMenuBus, Bincode> path="/rmenu_service.js">
            <ContextProvider<Rc<ClientContext>> context={context}>
                <div class="h-screen grid grid-rows-12">
                    <NavbarHOC/>
                    <div class="row-span-11 overflow-y-auto">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
                <RightMenuHOC />
                <ToastHOC />
                <ModalHOC />
            </ContextProvider<Rc<ClientContext>>>
            </WorkerProvider<RMenuBus, Bincode>>
            </WorkerProvider<ModalBus, Bincode>>
            </ReactorProvider<ChatReactor>>
        </BrowserRouter>
    })
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Suspense fallback={html!{<Loading/>}}>
            <ContextualApp/>
        </Suspense>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
