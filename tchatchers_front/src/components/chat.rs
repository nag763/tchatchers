// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use chrono::{DateTime, Datelike, Timelike, Utc};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::{WsMessageContent, WsReceptionStatus};
use yew::{function_component, html, Component, Context, Html, Properties};

const DEFAULT_PFP: &str = "/assets/no_pfp.webp";

#[derive(Properties, PartialEq)]
struct ProfilePictureProperties {
    #[prop_or(DEFAULT_PFP.into())]
    pub pfp: String,
    pub author: String,
    #[prop_or(true)]
    pub display_pfp: bool,
}

#[function_component(ProfilePicture)]
fn profile_picture(profile_picture_properties: &ProfilePictureProperties) -> Html {
    let class = match profile_picture_properties.display_pfp {
        true => "flex-shrink-0 h-10 w-10 rounded-full bg-gray-300",
        false => "flex-shrink-0 h-10 w-10 rounded-full bg-gray-300 invisible",
    };
    html! {
        <div>
            <div {class} title={profile_picture_properties.author.clone()}>
                <img class="h-10 w-10 rounded-full" src={profile_picture_properties.pfp.clone()} alt="No img"/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct MessageProperties {
    pub content: String,
    pub timestamp: DateTime<Utc>,
    #[prop_or_default]
    pub is_user: bool,
    pub reception_status: WsReceptionStatus,
}

#[function_component(Message)]
fn message(message_properties: &MessageProperties) -> Html {
    let timestamp = &message_properties.timestamp;
    let title: String = format!(
        "on {:02}/{:02}/{} at {}:{:02}",
        timestamp.day(),
        timestamp.month(),
        timestamp.year(),
        timestamp.hour(),
        timestamp.minute()
    );
    let class: &str = match message_properties.is_user {
        true => {
            "relative bg-blue-600 text-white p-3 rounded-l-lg rounded-br-lg mb-2 text-sm break-when-needed"
        }
        false => "relative bg-gray-300 mb-2 p-3 rounded-r-lg rounded-bl-lg text-sm break-when-needed",
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
    html! {
        <p {title} class="flex">
            // <span class="mb-2 flex flex-col-reverse" hidden={reception_checkmark.is_none()}>
            //     {reception_checkmark}
            // </span>
            <span {class}>
            {message_properties.content.as_str()}
                <span class="absolute right-0 bottom-0 pb-1 pr-1">
                {reception_checkmark}
                </span>
            </span>
        </p>
    }
}

#[derive(Properties, PartialEq)]
struct UserChatProperties {
    pub content: String,
    pub timestamp: DateTime<Utc>,
    #[prop_or_default]
    pub is_user: bool,
    pub author: String,
    #[prop_or("/assets/no_pfp.webp".into())]
    pub pfp: String,
    #[prop_or(true)]
    pub display_pfp: bool,
    pub reception_status: WsReceptionStatus,
}

#[function_component(UserChat)]
fn user_chat(user_chat_properties: &UserChatProperties) -> Html {
    let class = match user_chat_properties.is_user {
        true => "flex flex-row-reverse w-full mt-2 space-x-3 space-x-reverse max-w-xs ml-auto px-3",
        false => "flex w-full mt-2 space-x-3 max-w-xs px-3",
    };
    html! {
        <div {class}>
            <ProfilePicture pfp={user_chat_properties.pfp.clone()} author={user_chat_properties.author.clone()} display_pfp={user_chat_properties.display_pfp} />
            <Message reception_status={user_chat_properties.reception_status} content={user_chat_properties.content.clone()} is_user={user_chat_properties.is_user} timestamp={user_chat_properties.timestamp} />
        </div>
    }
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub messages: Vec<WsMessageContent>,
    pub room: String,
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
        let current_user_id = ctx.props().user.id;
        while let Some(current_element) = std::mem::replace(&mut next_element_opt, iterator.next())
        {
            let display_pfp = match next_element_opt {
                Some(next_element) => next_element.author.id != current_element.author.id,
                // Chat list is built in reverse order, so last built element is actually the first message,
                // so we display the pfp for the first message
                _ => true,
            };
            html_content.push(html! { <UserChat pfp={current_element.author.pfp.clone().unwrap_or_else(|| DEFAULT_PFP.into())} reception_status={current_element.reception_status} content={current_element.content.clone()} author={current_element.author.name.clone()} is_user={current_element.author.id == current_user_id} timestamp={current_element.timestamp} {display_pfp}/> });
        }
        html_content.into_iter().collect::<Html>()
    }
}
