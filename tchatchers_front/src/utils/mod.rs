// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

pub mod client_context;
pub mod keyed_list;
pub mod language;
pub mod requester;

pub fn get_ws_room_address(room: &str) -> String {
    let location = web_sys::window().unwrap().location();
    let host = location.host().unwrap();

    let protocol = location.protocol().unwrap();
    let ws_protocol = match protocol.as_str() {
        "https:" => "wss:",
        _ => "ws:",
    };
    format!(
        "{}//{}/ws/{}?_={}",
        ws_protocol,
        host,
        room,
        js_sys::Date::new_0().get_time()
    )
}
