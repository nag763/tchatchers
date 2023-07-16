// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The locale is corresponds to a reference that can be used to translate some labels to the user.
//!
//! The default locale is considered to be the english one, but since every user can use his language,
//! this entity provides the tools to translates the app while browsing.

use std::{collections::{HashMap}, sync::OnceLock};

use serde::{Deserialize, Serialize};

static LOCALES : OnceLock<HashMap<i32, Locale>> = OnceLock::new();

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Translation(HashMap<String, String>);

impl std::ops::Deref for Translation {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Translation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Translation {
    pub fn get_or_default(&self, label: &str, default: &str) -> String {
        match self.get(label) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }
}

/// The locale is a reference to translate the application.
///
/// A locale inherits usually from a language, and is more specific
/// to match a subgroup of a language users (ie french speaking canadians).
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Locale {
    /// In base id.
    pub id: i32,
    /// The locale shortname.
    pub short_name: String,
    /// The locale's longname.
    pub long_name: String,
    /// Translations associated to the locale.
    pub translations: Translation
}

impl Locale {

    fn init_cell() -> HashMap<i32, Locale> {
        serde_yaml::from_str(include_str!("config/translations.yml")).unwrap()
    }

    /// Returns all the locales sorted alphabeticly by their name.
    pub fn find_by_id(id: i32) -> Option<Locale> {
        let result = LOCALES.get_or_init(Self::init_cell);
        result.get(&id).cloned()
    }

    pub fn get_available_locales() -> Vec<Locale> {
        LOCALES.get_or_init(Self::init_cell).values().cloned().collect()
    }

}
