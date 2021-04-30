use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/#login"]
    Login,
    #[to = "/#register"]
    Register,
    #[to = "/#content"]
    Content,
    #[to = "/"]
    Home,
}
