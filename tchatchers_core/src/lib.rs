//! This crate is used to define the common structs and data used both by
//! the client and the server applications.

#[cfg(any(feature = "back", feature = "async", feature = "cli"))]
pub mod async_message;
pub mod authorization_token;
pub(crate) mod common;
pub mod locale;
#[cfg(feature = "front")]
pub mod navlink;
#[cfg(any(feature = "back", feature = "cli", feature = "async"))]
pub mod pool;
pub mod profile;
pub mod refresh_token;
pub mod report;
pub mod room;
pub mod serializable_token;
pub mod user;
pub mod validation_error_message;
pub mod ws_message;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
