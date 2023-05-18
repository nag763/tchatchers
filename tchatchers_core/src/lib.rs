//! This crate is used to define the common structs and data used both by
//! the client and the server applications.

pub mod app_context;
pub mod authorization_token;
pub(crate) mod common;
pub mod locale;
#[cfg(feature = "back")]
pub mod manager;
pub mod navlink;
#[cfg(any(feature = "back", feature = "cli"))]
pub mod pool;
pub mod profile;
pub mod refresh_token;
pub mod reported_message;
pub mod reported_user;
pub mod room;
pub mod serializable_token;
pub mod translation;
pub mod user;
pub mod validation_error_message;
pub mod ws_message;

#[macro_use]
extern crate lazy_static;
