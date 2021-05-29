use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::{
    agent::Bridged, format::Json, html, services::StorageService, Bridge, Component, ComponentLink,
};
use yew_material::{top_app_bar_fixed::MatTopAppBarTitle, MatTopAppBarFixed};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::{
    pages::{
        default::DefaultComponent, device_content::DeviceContent, home::HomeComponent,
        login::LoginComponent, logout_hint::LogoutHint, modify_device::ModifyDevice,
        register::RegisterComponent,
    },
    route::AppRoute,
};

pub struct App {
    link: ComponentLink<Self>,
    state: State,
    storage: StorageService,
    route_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Default)]
struct State {
    login_token: String,
    mail: String,
    name: String,
    is_logged_in: bool,
    device_id: String,
    device_name: String,
    device_info: String,
}

pub enum Msg {
    Nop,
    Login((String, String, String)),
    Logout,
    Register,
    SelectDevice((String, String, String)),
}

const STORAGE_KEY: &str = "pepcy.device_viewer";

#[derive(Deserialize, Serialize)]
struct StoredData {
    login_token: String,
    mail: String,
    name: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(yew::services::storage::Area::Local)
            .expect("Failed to init storage");
        let mut state = State::default();
        if let Json(Ok(StoredData {
            login_token,
            mail,
            name,
        })) = storage.restore(STORAGE_KEY)
        {
            state.login_token = login_token;
            state.mail = mail;
            state.name = name;
        }
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        Self {
            link,
            state,
            storage,
            route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::Login((login_token, mail, name)) => {
                let data = StoredData {
                    login_token: login_token.clone(),
                    mail: mail.clone(),
                    name: name.clone(),
                };
                self.storage.store(STORAGE_KEY, Json(&data));

                self.state.is_logged_in = true;
                self.state.login_token = login_token;
                self.state.mail = mail;
                self.state.name = name;
                self.route_agent.send(ChangeRoute(AppRoute::Home.into()));
                true
            }
            Msg::Logout => {
                self.state.is_logged_in = false;
                self.state.mail = "".to_string();
                self.state.name = "".to_string();
                self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
            Msg::Register => {
                self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
            Msg::SelectDevice((id, name, info)) => {
                self.state.device_id = id;
                self.state.device_name = name;
                self.state.device_info = info;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let login_callback = self
            .link
            .callback(|data: (String, String, String)| Msg::Login(data));
        let register_callback = self.link.callback(|_| Msg::Register);
        let logout_callback = self.link.callback(|_| Msg::Logout);
        let select_device_callback = self
            .link
            .callback(|data: (String, String, String)| Msg::SelectDevice(data));

        let login_token = Rc::new(self.state.login_token.clone());
        let mail = Rc::new(self.state.mail.clone());
        let name = Rc::new(self.state.name.clone());
        let device_id = Rc::new(self.state.device_id.clone());
        let device_name = Rc::new(self.state.device_name.clone());
        let device_info = Rc::new(self.state.device_info.clone());

        html! {
            <div>
                <MatTopAppBarFixed>
                    <MatTopAppBarTitle>
                        <div class="app-title">
                            <h1>{ "Device Viewer" }</h1>
                        </div>
                    </MatTopAppBarTitle>
                </MatTopAppBarFixed>
                <Router<AppRoute, ()> render=Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Default => html! {
                            <DefaultComponent login_token=login_token.clone() />
                        },
                        AppRoute::Login => html! {
                            <LoginComponent onlogin=login_callback.clone() />
                        },
                        AppRoute::Register => html! {
                            <RegisterComponent onregister=register_callback.clone() />
                        },
                        AppRoute::Home => html! {
                            <HomeComponent
                                login_token=login_token.clone()
                                mail=mail.clone()
                                name=name.clone()
                                onlogout=logout_callback.clone()
                                onselect=select_device_callback.clone() />
                        },
                        AppRoute::ModifyDevice => html! {
                            <ModifyDevice
                                login_token=login_token.clone()
                                mail=mail.clone()
                                id=device_id.clone()
                                name=device_name.clone()
                                info=device_info.clone() />
                        },
                        AppRoute::DeviceContent => html! {
                            <DeviceContent
                                login_token=login_token.clone()
                                mail=mail.clone()
                                id=device_id.clone()
                                name=device_name.clone()
                                info=device_info.clone() />
                        },
                        AppRoute::LogoutHint => html! {
                            <LogoutHint onlogout=logout_callback.clone() />
                        }
                    }
                }) />
            </div>
        }
    }
}
