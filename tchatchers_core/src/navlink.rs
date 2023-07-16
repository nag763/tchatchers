// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

//! A navlink is a reference to a front end page that is stored in the database.
//!
//! This module is mainly used to differentiate the privilegied accesses that can be existing between the different user types.
//!
//! For instance, an admin usually do not have access to the same screens as a simple user.
//!
//! This difference is stored in the database, and then returned to the client once he logs in.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::profile::Profile;

static NAVLINKS: OnceLock<Vec<Navlink>> = OnceLock::new();

/// A navlink is a reference to a front-end page.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Navlink {
    /// In base id.
    pub id: i32,
    /// Label associated to the navlink.
    pub label: String,
    /// The hyper reference.
    pub href: String,
    /// The default translation of the label.
    pub default_translation: String,
    /// Visibility, who can access this label.
    pub visibility: Vec<Profile>,
}

impl Navlink {
    fn init_cell() -> Vec<Navlink> {
        serde_yaml::from_str(include_str!("config/navlink.yml")).unwrap()
    }

    pub fn get_visibility_for_profile(profile: &Profile) -> Vec<Navlink> {
        NAVLINKS
            .get_or_init(Self::init_cell)
            .iter()
            .filter(|navlink| navlink.visibility.contains(profile))
            .cloned()
            .collect()
    }
}
