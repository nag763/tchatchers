use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "back", derive(sqlx::FromRow))]
pub struct Timezone {
    pub tz_name: String,
    pub tz_offset: i64,
}
