use serde::Deserialize;

use super::AsyncQueue;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub queue: AsyncQueue,
    pub interval: u64,
}
