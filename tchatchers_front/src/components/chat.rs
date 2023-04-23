// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::{WsMessageContent, WsReceptionStatus};
use uuid::Uuid;
use web_sys::MouseEvent;
use yew::{function_component, html, use_state, AttrValue, Component, Context, Html, Properties};
use yew_agent::Dispatched;

use crate::components::right_menu::message_rmenu::MessageRMenuProps;
use crate::components::right_menu::profile_rmenu::ProfileRMenuProps;
use crate::components::right_menu::RMenuKind;
use crate::services::rmenu_bus::{RMenuBus, RMenusBusEvents};

const DEFAULT_PFP: &str = "/assets/no_pfp.webp";

#[derive(Properties, PartialEq)]
struct ProfilePictureProperties {
    #[prop_or(DEFAULT_PFP.into())]
    pub pfp: AttrValue,
    pub author: AttrValue,
    #[prop_or(true)]
    pub display_pfp: bool,
    pub author_id: i32,
    pub is_self: bool,
}

#[function_component(ProfilePicture)]
fn profile_picture(profile_picture_properties: &ProfilePictureProperties) -> Html {
    let class = match profile_picture_properties.display_pfp {
        true => "flex-shrink-0 h-10 w-10 rounded-full bg-gray-300",
        false => "flex-shrink-0 h-10 w-10 rounded-full bg-gray-300 invisible",
    };

    let user_id = profile_picture_properties.author_id;
    let is_self = profile_picture_properties.is_self;

    html! {
        <div>
            <div {class} title={profile_picture_properties.author.clone()} oncontextmenu={move |me: MouseEvent|
                {
                    if !is_self {
                        me.prevent_default();
                        RMenuBus::dispatcher().send(RMenusBusEvents::OpenRMenu(me.client_x(), me.client_y(), RMenuKind::ProfileRMenu(ProfileRMenuProps{ user_id })));
                    }
                }}>
                <img class="h-10 w-10 rounded-full" src={profile_picture_properties.pfp.clone()} alt="No img"/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MessageProperties {
    pub content: AttrValue,
    pub timestamp: DateTime<Utc>,
    pub uuid: Uuid,
    #[prop_or_default]
    pub is_user: bool,
    pub reception_status: WsReceptionStatus,
}

#[function_component(Message)]
fn message(message_properties: &MessageProperties) -> Html {
    let timestamp = &message_properties.timestamp;
    let title: AttrValue = format!(
        "on {:02}/{:02}/{} at {}:{:02}",
        timestamp.day(),
        timestamp.month(),
        timestamp.year(),
        timestamp.hour(),
        timestamp.minute()
    )
    .into();
    let class: &str = match message_properties.is_user {
        true => {
            "relative bg-blue-600 text-white p-3 rounded-l-lg rounded-br-lg mb-2 text-sm break-when-needed max-w-xs"
        }
        false => "relative bg-gray-300 mb-2 p-3 rounded-r-lg rounded-bl-lg text-sm break-when-needed max-w-xs",
    };
    let reception_checkmark = match message_properties.reception_status {
        WsReceptionStatus::Sent if message_properties.is_user => Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-2 h-2">
             <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
            </svg>

        }),
        WsReceptionStatus::Seen if message_properties.is_user => Some(html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-2 h-2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>

        }),
        _ => None,
    };
    let div_class = match message_properties.is_user {
        true => "flex ",
        false => "flex flex-row-reverse",
    };

    let message_id = message_properties.uuid;
    let is_self = message_properties.is_user;

    let hide_timestamp = use_state(|| true);
    html! {
        <div id={message_id.to_string()} class={div_class}>
            <small hidden={*hide_timestamp} class="dark:text-white mx-2">{&title}</small>
            <p {title} class={class} onclick={move |_me| hide_timestamp.set(!*hide_timestamp)} oncontextmenu={move |me: MouseEvent|
                {
                    me.prevent_default();
                    RMenuBus::dispatcher().send(RMenusBusEvents::OpenRMenu(me.client_x(), me.client_y(), RMenuKind::MessageRMenu(MessageRMenuProps{ message_id, is_self })));
                }}
            >
                {message_properties.content.as_str()}
                    <span class="absolute right-0 bottom-0 pb-1 pr-1">
                    {reception_checkmark}
                    </span>
            </p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserChatProperties {
    pub content: AttrValue,
    pub uuid: Uuid,
    pub timestamp: DateTime<Utc>,
    #[prop_or_default]
    pub is_user: bool,
    pub author: AttrValue,
    #[prop_or("/assets/no_pfp.webp".into())]
    pub pfp: AttrValue,
    #[prop_or(true)]
    pub display_pfp: bool,
    pub reception_status: WsReceptionStatus,
    pub author_id: i32,
}

#[function_component(UserChat)]
fn user_chat(user_chat_properties: &UserChatProperties) -> Html {
    let mut class: String = match user_chat_properties.is_user {
        true => "flex flex-row-reverse w-full space-x-3 space-x-reverse ml-auto px-3".into(),
        false => "flex w-full space-x-3 px-3".into(),
    };
    if user_chat_properties.display_pfp {
        class += " mt-3";
    }
    html! {
        <div {class}>
            <ProfilePicture pfp={user_chat_properties.pfp.clone()} author={user_chat_properties.author.clone()} display_pfp={user_chat_properties.display_pfp} author_id={user_chat_properties.author_id} is_self={user_chat_properties.is_user}/>
            <Message uuid={user_chat_properties.uuid} reception_status={user_chat_properties.reception_status} content={user_chat_properties.content.clone()} is_user={user_chat_properties.is_user} timestamp={user_chat_properties.timestamp} />
        </div>
    }
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub messages: Vec<WsMessageContent>,
    pub room: AttrValue,
    pub user: PartialUser,
}

pub struct Chat;

impl Component for Chat {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut iterator = ctx.props().messages.iter();
        let mut next_element_opt = iterator.next();
        let mut html_content: Vec<Html> = Vec::with_capacity(ctx.props().messages.len());
        let user_offset =
            Duration::minutes(-(js_sys::Date::new_0().get_timezone_offset().round() as i64));
        let current_user_id = ctx.props().user.id;
        while let Some(current_element) = std::mem::replace(&mut next_element_opt, iterator.next())
        {
            let display_pfp = match next_element_opt {
                Some(next_element) => next_element.author.id != current_element.author.id,
                // Chat list is built in reverse order, so last built element is actually the first message,
                // so we display the pfp for the first message
                _ => true,
            };
            html_content.push(html! { <UserChat uuid={current_element.uuid} pfp={current_element.author.pfp.clone().unwrap_or_else(|| DEFAULT_PFP.into())} reception_status={current_element.reception_status} content={current_element.content.clone()} author_id={current_element.author.id} author={current_element.author.name.clone()} is_user={current_element.author.id == current_user_id} timestamp={current_element.timestamp + user_offset} {display_pfp}/> });
        }
        html_content.into_iter().collect::<Html>()
    }
}
