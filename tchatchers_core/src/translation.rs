#[cfg(feature = "back")]
use crate::manager::ManagerError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

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

    #[cfg(feature = "front")]
    pub fn get_or_default(&self, label: &str, default: &str) -> String {
        match self.get(label) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }
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

    pub fn get_all_translations(&self) -> Result<HashMap<i32, Translation>, ManagerError<i32>> {
        if !self.init {
            Err(ManagerError::NotInit)
        } else {
            Ok(self.translations.clone())
        }
    }
}
