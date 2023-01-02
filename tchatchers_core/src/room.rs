//! Rooms are where user chats with each others.
//!
//! They are persisted within redis as "room_name=redis_key", so that any user
//! that reconnects retieve the messages sent before he joined.

// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use crate::common::RE_LIMITED_CHARS;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct RoomNameValidator {
    #[validate(
        length(min = 1, max = 128),
        regex(path = "RE_LIMITED_CHARS", code = "limited_chars")
    )]
    name: String,
}

impl From<String> for RoomNameValidator {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}
