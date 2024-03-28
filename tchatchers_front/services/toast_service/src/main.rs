// Copyright ⓒ 2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/tchatchers/blob/main/LICENSE.MD).

use toast_service::ToastBus;
use yew_agent::Registrable;

fn main() {
    ToastBus::registrar().register();
}
