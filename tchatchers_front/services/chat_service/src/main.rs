// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use chat_service::bus::ChatBus;
use yew_agent::PublicWorker;

fn main() {
    ChatBus::register();
}
