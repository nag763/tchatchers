// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

/// The event bus is used to transmit the content of websocket messages to the
/// components.
pub mod chat_bus;
/// This component connects to the backend through websockets in order to chat
/// with other users.
pub mod chat_service;
/// Event Bus used to display toasts.
pub mod toast_bus;

pub mod modal_bus;

pub mod rmenu_bus;
