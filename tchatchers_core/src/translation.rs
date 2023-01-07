use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

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


    #[cfg(feature = "back")]
    async fn get_locales(pool: &sqlx::PgPool) -> Vec<(i32,)> {
        sqlx::query_as("SELECT loc.id FROM LOCALE loc")
            .fetch_all(pool)
            .await
            .unwrap()
    }


    #[cfg(feature = "front")]
    pub fn get_or_default(self, label: &str, default: &str) -> String {
        match self.get(label) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }
}

#[cfg(feature = "back")]
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Display)]
pub enum TranslationManagerError {
    #[display(fmt = "The translation manager hasn't been init.")]
    NotInit,
    #[display(fmt = "The locale hasn't been found, please use an existing locale.")]
    LocaleDoesNotExist,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg(feature = "back")]
pub struct TranslationManager {
    init: bool,
    translations: HashMap<i32, Translation>,
}

#[cfg(feature = "back")]
impl TranslationManager {
    pub async fn init(pool: &sqlx::PgPool) -> TranslationManager {
        let locales: Vec<(i32,)> = Translation::get_locales(pool).await;
        let mut translations: HashMap<i32, Translation> =
            HashMap::with_capacity(locales.capacity());
        for (locale,) in locales {
            translations.insert(
                locale,
                Translation::get_translations_for_locale(locale, pool).await,
            );
        }
        TranslationManager {
            init: true,
            translations,
        }
    }

    pub async fn get_translations_for_locale(
        &self,
        locale_id: i32,
    ) -> Result<Translation, TranslationManagerError> {
        if !self.init {
            return Err(TranslationManagerError::NotInit);
        }
        let Some(translation) = self.translations.get(&locale_id) else {
            return Err(TranslationManagerError::LocaleDoesNotExist);
        };
        Ok(translation.clone())
    }
}
