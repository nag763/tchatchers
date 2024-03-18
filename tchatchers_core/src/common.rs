// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

lazy_static! {
    pub static ref REFRESH_TOKEN_EXPIRACY_TIME: chrono::Duration =
        chrono::Duration::try_weeks(1).unwrap();
    pub static ref AUTHORIZATION_TOKEN_EXPIRACY_TIME: chrono::Duration =
        chrono::Duration::try_minutes(15).unwrap();
}

pub(crate) fn limited_chars_checker(room_name: &str) -> Result<(), validator::ValidationError> {
    for c in room_name.chars() {
        if !matches!(c.to_ascii_lowercase(), 'a'..='z' | '1'..='9' | '_') {
            return Err(validator::ValidationError::new("limited_chars"));
        }
    }
    Ok(())
}
