#[cfg(feature = "back")]
use crate::{manager::ManagerError, profile::Profile};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Navlink {
    pub id: i32,
    pub label: String,
    pub href: String,
    pub default_translation: String,
    pub before: Option<i32>,
}

#[cfg(feature = "back")]
impl Navlink {
    async fn get_for_profile(profile: Profile, pg_pool: &sqlx::PgPool) -> Vec<Navlink> {
        sqlx::query_as(
            "
            SELECT nv.id, name as label, href, default_translation, before
            FROM NAVLINK nv 
            INNER JOIN NAVLINK_PROFILE np ON np.navlink_id=nv.id 
            INNER JOIN LABEL lbl ON lbl.id = nv.label_id 
            WHERE np.profile_id=$1
        ",
        )
        .bind(profile as i32)
        .fetch_all(pg_pool)
        .await
        .unwrap()
    }

    async fn get_for_profile_sorted(profile: Profile, pg_pool: &sqlx::PgPool) -> Vec<Navlink> {
        let mut sortable_navlink = Self::get_for_profile(profile, pg_pool).await;
        sortable_navlink.sort_by(|a, b| b.before.cmp(&a.before));
        sortable_navlink
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg(feature = "back")]
pub struct NavlinkManager {
    init: bool,
    navlinks: HashMap<Profile, Vec<Navlink>>,
}

#[cfg(feature = "back")]
impl NavlinkManager {
    pub async fn init(pg_pool: &sqlx::PgPool) -> NavlinkManager {
        let profiles = Profile::iterator();
        let mut hashmap: HashMap<Profile, Vec<Navlink>> = HashMap::new();
        for profile in profiles {
            hashmap.insert(
                profile,
                Navlink::get_for_profile_sorted(profile, pg_pool).await,
            );
        }
        NavlinkManager {
            init: true,
            navlinks: hashmap,
        }
    }

    pub fn get_navlink_for_profile(
        &self,
        profile: Profile,
    ) -> Result<Vec<Navlink>, ManagerError<Profile>> {
        if !self.init {
            return Err(ManagerError::NotInit);
        }
        let Some(navlinks) = self.navlinks.get(&profile) else {
            return Err(ManagerError::NotBound(profile));
        };
        Ok(navlinks.clone())
    }

    pub fn get_navlinks(&self) -> Result<HashMap<Profile, Vec<Navlink>>, ManagerError<Profile>> {
        if !self.init {
            return Err(ManagerError::NotInit);
        }
        Ok(self.navlinks.clone())
    }
}
