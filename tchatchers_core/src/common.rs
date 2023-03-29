// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use regex::Regex;

lazy_static! {
    pub static ref RE_LIMITED_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9-_]*$").unwrap();
    pub static ref REFRESH_TOKEN_EXPIRACY_TIME: chrono::Duration = chrono::Duration::weeks(1);
    pub static ref AUTHORIZATION_TOKEN_EXPIRACY_TIME: chrono::Duration =
        chrono::Duration::minutes(15);
}
