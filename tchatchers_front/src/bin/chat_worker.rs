// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use tchatchers_front::services::event_bus::EventBus;
use yew_agent::PublicWorker;

fn main() {
    EventBus::register();
}
