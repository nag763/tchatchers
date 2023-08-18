//! The locale corresponds to a reference that can be used to translate some labels to the user.
//!
//! The default locale is considered to be the English one, but since every user can use their own language,
//! this entity provides the tools to translate the app while browsing.

use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};

static LOCALES: OnceLock<HashMap<i32, Locale>> = OnceLock::new();

/// A collection of translations for labels.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
struct Translations {
    #[serde(rename = "translations")]
    locales: Vec<Locale>,
}

/// A collection of translations for labels.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TranslationMap(HashMap<String, String>);

impl std::ops::Deref for TranslationMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TranslationMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TranslationMap {
    /// Gets the translation for the specified label. Returns the default value if no translation is found.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to translate.
    /// * `default` - The default value to return if no translation is found.
    pub fn get_or_default(&self, label: &str, default: &str) -> String {
        match self.get(label) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }
}

/// The locale is a reference used for application translation.
///
/// A locale inherits usually from a language, and is more specific
/// to match a subgroup of a language users (ie french-speaking Canadians).
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Locale {
    /// In base id.
    pub id: i32,
    /// The locale short name.
    pub short_name: String,
    /// The locale's long name.
    pub long_name: String,
    /// Web locale values.
    pub web_names: Vec<String>,
    /// Translations associated with the locale.
    pub translation_map: TranslationMap,
}

impl Locale {
    /// Initializes the HashMap of locales.
    fn init_cell() -> HashMap<i32, Locale> {
        let translations: Translations =
            serde_yaml::from_str(include_str!("config/translations.yml")).unwrap();
        let mut cell_value = HashMap::new();
        for locale in translations.locales {
            let locale_id = locale.id;
            cell_value.insert(locale_id, locale);
        }
        cell_value
    }

    /// Returns the locale with the specified ID, if found.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the locale to find.
    pub fn find_by_id(id: i32) -> Option<Locale> {
        let result = LOCALES.get_or_init(Self::init_cell);
        result.get(&id).cloned()
    }

    /// Returns a list of all available locales.
    pub fn get_available_locales() -> Vec<Locale> {
        LOCALES
            .get_or_init(Self::init_cell)
            .values()
            .cloned()
            .collect()
    }

    pub fn get_for_web_names(web_names: Vec<String>) -> Option<Locale> {
        let locales: Vec<Locale> = LOCALES
            .get_or_init(Self::init_cell)
            .values()
            .cloned()
            .collect();
        for web_name in web_names {
            if let Some(index) = locales.iter().position(|v| v.web_names.contains(&web_name)) {
                return locales.get(index).cloned();
            }
        }
        None
    }

    pub fn get_default_locale() -> Locale {
        let locales = LOCALES.get_or_init(Self::init_cell);
        locales.get(&1i32).cloned().unwrap()
    }

    pub fn get_default_translation(label: &str) -> Option<String> {
        let locales = LOCALES.get_or_init(Self::init_cell);
        let default_locale = locales.get(&1)?;
        default_locale.translation_map.get(label).cloned()
    }

    pub fn get_keyed_list() -> Vec<(i32, String)> {
        let locales = LOCALES.get_or_init(Self::init_cell);
        locales
            .iter()
            .map(|(key, locale)| (*key, locale.long_name.clone()))
            .collect()
    }
}
