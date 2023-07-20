pub fn get_navigator_languages() -> Option<Vec<String>> {
    let navigator_language = web_sys::window()?.navigator().languages();
    Some(
        navigator_language
            .into_iter()
            .filter_map(|val| val.as_string())
            .collect(),
    )
}
