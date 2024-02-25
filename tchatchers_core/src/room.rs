// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! Rooms are where user chats with each others.
//!
//! They are persisted within redis as "room_name=redis_key", so that any user
//! that reconnects retieve the messages sent before he joined.

use validator::Validate;
use crate::common::limited_chars_checker;

#[derive(Debug, Validate)]
pub struct RoomNameValidator {
    #[validate(
        length(min = 1, max = 128),
        custom(
            function = "limited_chars_checker",
            code = "limited_chars"
        )
    )]
    name: String,
}

impl From<String> for RoomNameValidator {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}
