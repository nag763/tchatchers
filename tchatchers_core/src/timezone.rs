// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The timezones is a named offset from the london time.
//! 
//! It is used to change the message's timestamp when the user is logged on the app.

use serde::{Deserialize, Serialize};

/// A timezone is a constant difference from the London's time.
#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Timezone {
    /// The timezone name.
    pub tz_name: String,
    /// Its offset to London's time.
    pub tz_offset: i64,
}
