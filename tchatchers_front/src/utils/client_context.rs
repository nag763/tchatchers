use tchatchers_core::app_context::UserContext;
use yew::UseStateHandle;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientContext {
    pub user_context: UseStateHandle<Option<UserContext>>,
    pub bearer: UseStateHandle<Option<String>>,
}
