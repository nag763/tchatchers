use modal_service::ModalBus;
use yew_agent::Registrable;

fn main() {
    ModalBus::registrar().register();
}
