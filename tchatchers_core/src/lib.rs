//! This crate is used to define the common structs and data used both by
//! the client and the server applications.

pub mod app_context;
pub(crate) mod common;
pub mod jwt;
pub mod locale;
#[cfg(feature = "back")]
pub mod manager;
pub mod navlink;
#[cfg(feature = "back")]
pub mod pool;
pub mod profile;
pub mod room;
pub mod timezone;
pub mod translation;
pub mod user;
pub mod validation_error_message;
pub mod ws_message;

#[macro_use]
extern crate lazy_static;
