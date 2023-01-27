// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! A navlink is a reference to a front end page that is stored in the database.
//! 
//! This module is mainly used to differentiate the privilegied accesses that can be existing between the different user types.
//! 
//! For instance, an admin usually do not have access to the same screens as a simple user.
//! 
//! This difference is stored in the database, and then returned to the client once he logs in.

#[cfg(feature = "back")]
use crate::{manager::ManagerError, profile::Profile};
use serde::{Deserialize, Serialize};
#[cfg(feature = "back")]
use std::collections::HashMap;

/// A navlink is a reference to a front-end page. 
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Navlink {
    /// In base id.
    pub id: i32,
    /// Label associated to the navlink.
    pub label: String,
    /// The hyper reference.
    pub href: String,
    /// The default translation of the label.
    pub default_translation: String,
    /// Ordering, before which other label the entity should appear.
    pub before: Option<i32>,
}

#[cfg(feature = "back")]
impl Navlink {
    
    /// Returns the navlinks for the profile.
    /// 
    /// # Arguments 
    /// 
    /// - profile : the user's associated profile.
    /// - pg_pool : the connection pool.
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

    /// Returns the navlinks ordered for a given profile.
    /// 
    /// # Arguments 
    /// 
    /// - profile : the user's associated profile.
    /// - pg_pool : the connection pool.
    async fn get_for_profile_sorted(profile: Profile, pg_pool: &sqlx::PgPool) -> Vec<Navlink> {
        let mut sortable_navlink = Self::get_for_profile(profile, pg_pool).await;
        sortable_navlink.sort_by(|a, b| b.before.cmp(&a.before));
        sortable_navlink
    }
}

/// Manager used to cache server side the navlinks for each profile.
#[derive(Clone, Debug, PartialEq)]
#[cfg(feature = "back")]
pub struct NavlinkManager {
    /// Whether the manager has been initialized.
    init: bool,
    /// Navlink mapping per profile.
    navlinks: HashMap<Profile, Vec<Navlink>>,
}

#[cfg(feature = "back")]
impl NavlinkManager {

    /// Initialize the manager
    /// 
    /// # Arguments
    /// 
    /// - pg_pool : The pool to cache the manager.
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

    /// Returns an ordered navlink list for the given profile.
    /// 
    /// # Arguments
    /// 
    /// - profile : The profile to get the navlinks from. 
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

    /// Returns all the navlinks cached in the manager.
    pub fn get_navlinks(&self) -> Result<HashMap<Profile, Vec<Navlink>>, ManagerError<Profile>> {
        if !self.init {
            return Err(ManagerError::NotInit);
        }
        Ok(self.navlinks.clone())
    }
}
