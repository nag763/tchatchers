use tchatchers_core::user::PartialUser;
use wasm_bindgen::JsCast;

fn get_jwt_public_part(jwt: &str) -> PartialUser {
    let mut splits = jwt.split('.');
    let public_part_encoded = splits.nth(1).unwrap();
    let decoded_bytes: Vec<u8> = base64::decode(public_part_encoded).unwrap();
    let binding = decoded_bytes.clone();
    let decoded: &str = std::str::from_utf8(&binding).unwrap();
    let value: serde_json::value::Value = serde_json::from_str(&decoded).unwrap();
    let user = serde_json::from_value(value["user"].clone()).unwrap();
    user
}

pub fn get_user() -> Result<PartialUser, &'static str> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    let document_cookies = html_document.cookie().unwrap();
    let cookies = &mut document_cookies.split(';');
    let mut jwt_val: String = String::default();
    for cookie in cookies.by_ref() {
        if let Some(i) = cookie.find('=') {
            let (key, val) = cookie.split_at(i + 1);
            if key == "jwt=" {
                jwt_val = val.into();
            }
        }
    }
    if jwt_val != String::default() {
        Ok(get_jwt_public_part(&jwt_val))
    } else {
        Err("No JWT found")
    }
}
