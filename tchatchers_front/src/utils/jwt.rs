use tchatchers_core::user::PartialUser;

pub fn get_jwt_public_part(jwt: &str) -> PartialUser {
    let mut splits = jwt.split('.');
    let public_part_encoded = splits.nth(1).unwrap();
    let decoded_bytes: Vec<u8> = base64::decode(public_part_encoded).unwrap();
    let binding = decoded_bytes.clone();
    let decoded: &str = std::str::from_utf8(&binding).unwrap();
    let value: serde_json::value::Value = serde_json::from_str(&decoded).unwrap();
    let user = serde_json::from_value(value["user"].clone()).unwrap();
    user
}
