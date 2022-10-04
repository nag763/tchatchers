//! This crate is used to define the common structs and data used both by
//! the client and the server applications.

pub mod jwt;
#[cfg(feature = "back")]
pub mod pool;
#[cfg(feature = "back")]
pub mod room;
pub mod user;
pub mod ws_message;
