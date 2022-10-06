// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

/// The auth bus is a data bus used to indicate to the other components the
/// state of the user authentication.
pub mod auth_bus;
/// This component connects to the backend through websockets in order to chat
/// with other users.
pub mod chat;
/// The event bus is used to transmit the content of websocket messages to the
/// components.
pub mod event_bus;
/// Wrapper for messages received by the backend.
pub mod message;
/// Event Bus used to display toasts.
pub mod toast_bus;
