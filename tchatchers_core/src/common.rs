// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use regex::Regex;

lazy_static! {
    pub static ref RE_LIMITED_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9-_]*$").unwrap();
}
