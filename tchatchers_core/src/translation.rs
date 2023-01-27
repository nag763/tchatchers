// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! The translation module is mainly used to translate the app's text content given a user's locale.
//! 
//! This helps for the internationalization of the application. 

#[cfg(feature = "back")]
use crate::manager::ManagerError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// A translation is a set of label (key) for which correspond a translation (value).
/// 
/// A translation is built from a locale, if the locale doesn't have a translation for a given label,
/// then the default translation of the label will be used (in english).
/// 
/// ie (settings_menu_title) => "Settings" 
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Translation(HashMap<String, String>);

impl Deref for Translation {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Translation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Translation {
    
    /// Returns the translations for a given locale.
    /// 
    /// # Arguments
    /// 
    /// - locale_id : The id of the locale.
    /// - pool : The Postgres pool. 
    #[cfg(feature = "back")]
    async fn get_translations_for_locale(locale_id: i32, pool: &sqlx::PgPool) -> Self {
        let hashmap = sqlx::query_as("
            SELECT lbl.name, CASE WHEN tra.translation IS NULL THEN lbl.default_translation ELSE tra.translation END
            FROM LOCALE loc
            JOIN LABEL lbl ON TRUE
            LEFT OUTER JOIN TRANSLATION tra
            ON tra.label_id = lbl.id AND loc.id = tra.locale_id
            WHERE loc.id = $1
        ")
        .bind(locale_id)
        .fetch_all(pool)
        .await
        .unwrap().into_iter().collect();
        Translation(hashmap)
    }

    /// Returns the translation or the default argument if not found.
    /// 
    /// # Arguments
    /// 
    /// - label : The label to translation
    /// - default : The default translation if the label isn't translatable.
    #[cfg(feature = "front")]
    pub fn get_or_default(&self, label: &str, default: &str) -> String {
        match self.get(label) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }
}

/// Server Side cached translations.
/// 
/// This stores the locale id as key, and the corresponding translations as value.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg(feature = "back")]
pub struct TranslationManager {
    /// Whether this has been initialized.
    init: bool,
    /// The cached translations.
    translations: HashMap<i32, Translation>,
}

#[cfg(feature = "back")]
impl TranslationManager {
    
    /// Initializes the manager.
    /// 
    /// # Arguments
    /// 
    /// - pool : The postgres pool.
    pub async fn init(pool: &sqlx::PgPool) -> TranslationManager {
        use crate::locale::Locale;

        let locales: Vec<Locale> = Locale::get_all(pool).await;
        let mut translations: HashMap<i32, Translation> =
            HashMap::with_capacity(locales.capacity());
        for locale in locales {
            translations.insert(
                locale.id,
                Translation::get_translations_for_locale(locale.id, pool).await,
            );
        }
        TranslationManager {
            init: true,
            translations,
        }
    }

    /// Returns the translations for the given locale.
    ///
    /// # Argument
    /// 
    /// - locale_id: The locale for which we want the translations. 
    pub fn get_translations_for_locale(
        &self,
        locale_id: i32,
    ) -> Result<Translation, ManagerError<i32>> {
        if !self.init {
            return Err(ManagerError::NotInit);
        }
        let Some(translation) = self.translations.get(&locale_id) else {
            return Err(ManagerError::NotBound(locale_id));
        };
        Ok(translation.clone())
    }

    /// Returns all the translations available.
    pub fn get_all_translations(&self) -> Result<HashMap<i32, Translation>, ManagerError<i32>> {
        if !self.init {
            Err(ManagerError::NotInit)
        } else {
            Ok(self.translations.clone())
        }
    }
}
