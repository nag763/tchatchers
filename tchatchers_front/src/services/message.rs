// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WsBusMessageType {
    Error,
    NotConnected,
    Reply,
    KeepAlive,
    Ping,
    Pong,
    Closed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBusMessage {
    pub message_type: WsBusMessageType,
    pub content: String,
}
