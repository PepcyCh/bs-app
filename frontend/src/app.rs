use crate::fluent;
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::{
    agent::Bridged, classes, format::Json, html, services::StorageService, Bridge, Component,
    ComponentLink,
};
use yew_material::{
    list::{ListIndex, SelectedDetail},
    top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarTitle},
    MatIconButton, MatListItem, MatMenu, MatTopAppBarFixed, WeakComponentLink,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::{
    pages::{
        default::DefaultComponent, device_content::DeviceContent, home::HomeComponent,
        login::LoginComponent, logout_hint::LogoutHint, modify_device::ModifyDevice,
        register::RegisterComponent,
    },
    route::AppRoute,
};

static_loader! {
    static LOCALES = {
        locales: "./text/app_root",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct App {
    link: ComponentLink<Self>,
    lang_link: WeakComponentLink<MatMenu>,
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
    lang_id: LanguageIdentifier,
}

pub enum Msg {
    Nop,
    Login((String, String, String)),
    Logout,
    Register,
    ShowLanguageList,
    SelectLanguage(i32),
    SelectDevice((String, String, String)),
}

const STORAGE_KEY: &str = "pepcy.device_viewer";
const STORAGE_KEY_DEVICE: &str = "pepcy.device_viewer.device";
const STORAGE_KEY_LANG: &str = "pepcy.device_viewer.lang";

const LANG_LIST_ITEMS: [(&str, &str); 2] = [("简体中文", "zh-CN"), ("English", "en-US")];

#[derive(Deserialize, Serialize)]
struct StoredData {
    login_token: String,
    mail: String,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct StoredDeviceData {
    device_id: String,
    device_name: String,
    device_info: String,
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
        if let Json(Ok(StoredDeviceData {
            device_id,
            device_name,
            device_info,
        })) = storage.restore(STORAGE_KEY_DEVICE)
        {
            state.device_id = device_id;
            state.device_name = device_name;
            state.device_info = device_info;
        }
        let lang = if let Json(Ok(lang)) = storage.restore(STORAGE_KEY_LANG) {
            lang
        } else {
            "zh-CN".to_owned()
        };
        let lang_id: LanguageIdentifier = lang.parse().unwrap();
        state.lang_id = lang_id;

        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        Self {
            link,
            lang_link: WeakComponentLink::default(),
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
            Msg::ShowLanguageList => {
                self.lang_link.show();
                true
            }
            Msg::SelectLanguage(index) => {
                if index >= 0 && index < LANG_LIST_ITEMS.len() as i32 {
                    let lang = LANG_LIST_ITEMS[index as usize].1;
                    self.state.lang_id = lang.parse().unwrap();
                    self.storage
                        .store(STORAGE_KEY_LANG, Json(&lang.to_string()));
                    true
                } else {
                    false
                }
            }
            Msg::SelectDevice((id, name, info)) => {
                self.state.device_id = id.clone();
                self.state.device_name = name.clone();
                self.state.device_info = info.clone();

                let data = StoredDeviceData {
                    device_id: id,
                    device_name: name,
                    device_info: info,
                };
                self.storage.store(STORAGE_KEY_DEVICE, Json(&data));

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

        let lang_click = self.link.callback(|_| Msg::ShowLanguageList);
        let lang_select = self.link.callback(|e: SelectedDetail| {
            if let ListIndex::Single(Some(ind)) = e.index {
                Msg::SelectLanguage(ind as i32)
            } else {
                Msg::SelectLanguage(-1)
            }
        });

        let login_token = Rc::new(self.state.login_token.clone());
        let lang_id = self.state.lang_id.clone();
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
                            <h1>{ fluent!(self.state.lang_id, "header") }</h1>
                        </div>
                    </MatTopAppBarTitle>
                    <MatTopAppBarActionItems>
                        <div style="position:relative;">
                            <span onclick=lang_click>
                                <MatIconButton
                                    classes=classes!("translate-button")
                                    icon="translate" />
                            </span>
                            <MatMenu
                                quick=true
                                menu_link=self.lang_link.clone()
                                onselected=lang_select
                                absolute=true
                                x=-20
                                y=20 >
                                {
                                    for LANG_LIST_ITEMS
                                        .iter()
                                        .map(|item| {
                                            html! {
                                                <MatListItem>{ item.0 }</MatListItem>
                                            }
                                        })
                                }
                            </MatMenu>
                        </div>
                    </MatTopAppBarActionItems>
                </MatTopAppBarFixed>
                <Router<AppRoute, ()> render=Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Default => html! {
                            <DefaultComponent login_token=login_token.clone() />
                        },
                        AppRoute::Login => html! {
                            <LoginComponent
                                lang_id=lang_id.clone()
                                onlogin=login_callback.clone() />
                        },
                        AppRoute::Register => html! {
                            <RegisterComponent
                                lang_id=lang_id.clone()
                                onregister=register_callback.clone() />
                        },
                        AppRoute::Home => html! {
                            <HomeComponent
                                lang_id=lang_id.clone()
                                login_token=login_token.clone()
                                mail=mail.clone()
                                name=name.clone()
                                onlogout=logout_callback.clone()
                                onselect=select_device_callback.clone() />
                        },
                        AppRoute::ModifyDevice => html! {
                            <ModifyDevice
                                lang_id=lang_id.clone()
                                login_token=login_token.clone()
                                mail=mail.clone()
                                id=device_id.clone()
                                name=device_name.clone()
                                info=device_info.clone() />
                        },
                        AppRoute::DeviceContent => html! {
                            <DeviceContent
                                lang_id=lang_id.clone()
                                login_token=login_token.clone()
                                mail=mail.clone()
                                id=device_id.clone()
                                name=device_name.clone()
                                info=device_info.clone() />
                        },
                        AppRoute::LogoutHint => html! {
                            <LogoutHint
                                lang_id=lang_id.clone()
                                onlogout=logout_callback.clone() />
                        }
                    }
                }) />
            </div>
        }
    }
}
