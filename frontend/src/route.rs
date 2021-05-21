use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/#login"]
    Login,
    #[to = "/#register"]
    Register,
    #[to = "/#home"]
    Home,
    #[to = "/#modify_device"]
    ModifyDevice,
    #[to = "/#device_content"]
    DeviceContent,
    #[to = "/#go_to_login"]
    LogoutHint,
    #[to = "/"]
    Default,
}
