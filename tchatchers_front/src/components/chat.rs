// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).
use chrono::{Datelike, Timelike, DateTime, Utc};
use tchatchers_core::user::PartialUser;
use tchatchers_core::ws_message::{WsMessage, WsMessageType};
use yew::{html, Component, Context, Html, Properties, function_component};

const DEFAULT_PFP : &str = "/assets/no_pfp.webp";

#[derive(Properties, PartialEq)]
struct ProfilePictureProperties {
    #[prop_or(DEFAULT_PFP.into())]
    pub pfp: String,
    pub author: String,
}

#[function_component(ProfilePicture)]
fn profile_picture(profile_picture_properties: &ProfilePictureProperties) -> Html {
    html! {
        <div>
            <div class="flex-shrink-0 h-10 w-10 rounded-full bg-gray-300" title={profile_picture_properties.author.clone()}>
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
    pub is_user: bool
}

#[function_component(Message)]
fn message(message_properties: &MessageProperties) -> Html {
    let timestamp = &message_properties.timestamp;
    let title : String = format!("on {:02}/{:02}/{} at {}:{:02}", timestamp.day(), timestamp.month(), timestamp.year(), timestamp.hour(), timestamp.minute());
    let class : &str = match message_properties.is_user {
        true => "bg-blue-600 text-white p-3 rounded-l-lg rounded-br-lg mb-2",
        false => "bg-gray-300 mb-2 p-3 rounded-r-lg rounded-bl-lg",

    };
    html! {
        <div>
            <div {class} {title}>
                <p class="text-sm">{message_properties.content.as_str()}</p>
            </div>
        </div>
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
}

#[function_component(UserChat)]
fn user_chat(user_chat_properties: &UserChatProperties) -> Html {
    let class = match user_chat_properties.is_user {
        true => "flex flex-row-reverse w-full mt-2 space-x-3 space-x-reverse max-w-xs ml-auto px-3",
        false => "flex w-full mt-2 space-x-3 max-w-xs px-3"
    };
    html! { 
        <div {class}>
            <ProfilePicture pfp={user_chat_properties.pfp.clone()} author={user_chat_properties.author.clone()} />
            <Message content={user_chat_properties.content.clone()} is_user={user_chat_properties.is_user} timestamp={user_chat_properties.timestamp} />
        </div>
    }
}

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub messages: Vec<WsMessage>,
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
        html! {
            <>
            {ctx.props().messages
                .iter().map(|m| {
                match &m.message_type {
                    WsMessageType::Receive => {
                        if let (Some(author), Some(content)) = (m.author.clone(), m.content.clone()) {
                            html! { <UserChat pfp={author.pfp.unwrap_or_else(|| DEFAULT_PFP.into())} content={content} author={author.name.clone()} is_user={author.id == ctx.props().user.id} timestamp={m.timestamp}/> }
                        } else {
                            Html::default()
                        }
                    },
                    _ => html!{}
                }
            }).collect::<Html>()}
            </>
        }
    }
}
