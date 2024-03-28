use rmenu_service::RMenuBus;
use yew_agent::Registrable;

fn main() {
    RMenuBus::registrar().register();
}
