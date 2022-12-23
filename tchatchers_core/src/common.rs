use regex::Regex;

lazy_static! {
    pub static ref RE_LIMITED_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9-_]*$").unwrap();
}
