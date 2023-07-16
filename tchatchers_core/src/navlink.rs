/// A navlink is a reference to a front-end page that is stored in the database.
///
/// This module is mainly used to differentiate the privileged accesses that can exist
/// between different user types. For instance, an admin usually does not have access to
/// the same screens as a simple user. This difference is stored in the database and then
/// returned to the client once they log in.
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::profile::Profile;

static NAVLINKS: OnceLock<Vec<Navlink>> = OnceLock::new();

/// A navlink is a reference to a front-end page.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename = "navlink")]
pub struct Navlink {
    /// In base ID.
    pub id: i32,
    /// Label associated with the navlink.
    pub label: String,
    /// The hyperlink reference.
    pub href: String,
    /// The default translation of the label.
    pub default_translation: String,
    /// Visibility, specifying who can access this label.
    pub visibility: Vec<Profile>,
}

impl Navlink {
    fn init_cell() -> Vec<Navlink> {
        serde_yaml::from_str(include_str!("config/navlink.yml")).unwrap()
    }

    /// Returns the list of navlinks that are visible to the specified profile.
    ///
    /// # Arguments
    ///
    /// * `profile` - The profile for which to retrieve the visible navlinks.
    ///
    /// # Returns
    ///
    /// A vector of `Navlink` instances that are visible to the specified profile.
    pub fn get_visibility_for_profile(profile: &Profile) -> Vec<Navlink> {
        NAVLINKS
            .get_or_init(Self::init_cell)
            .iter()
            .filter(|navlink| navlink.visibility.contains(profile))
            .cloned()
            .collect()
    }
}
