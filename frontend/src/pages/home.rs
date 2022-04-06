use crate::{fluent, route::AppRoute, utils::card_div::CardDiv};
use common::{
    request::{
        CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceRequest, RemoveDeviceRequest,
    },
    response::{
        DeviceInfo, ErrorResponse, FetchDeviceListResponse, FetchDeviceResponse, SimpleResponse,
    },
};
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use std::{borrow::Cow, rc::Rc};
use yew::{
    agent::Bridged,
    classes,
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Bridge, Callback, Component, ComponentLink, InputData, Properties,
};
use yew_material::{MatButton, MatLinearProgress, MatTextField};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};

static_loader! {
    static LOCALES = {
        locales: "./text/home",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct HomeComponent {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    create_id: String,
    devices: Vec<DeviceInfo>,
    err: Option<String>,
}

pub enum Msg {
    Nop,
    Logout,
    LogoutRespone(SimpleResponse),
    ToLogin,
    EditCreateId(String),
    CreateDevice,
    CreateDeviceResponse(SimpleResponse),
    Fetch,
    FetchResponse(FetchDeviceListResponse),
    RemoveDevice(usize),
    RemoveDeviceResponse(SimpleResponse),
    Modify(usize),
    ModifyResponse(FetchDeviceResponse),
    Details(usize),
    DetialsResponse(FetchDeviceResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub lang_id: LanguageIdentifier,
    pub login_token: Rc<String>,
    pub mail: Rc<String>,
    pub name: Rc<String>,
    pub onlogout: Callback<()>,
    pub onselect: Callback<(String, String, String)>,
}

impl Component for HomeComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let mut component = Self {
            link,
            props,
            state: State::default(),
            route_agent,
            fetch_task: None,
        };
        if component.props.login_token.is_empty() {
            component.update(Msg::ToLogin);
        } else {
            component.update(Msg::Fetch);
        }
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::Logout => {
                self.state.err = None;
                let login_token = (*self.props.login_token).clone();
                crate::create_fetch_task!(self, "/api/logout", login_token, LogoutRespone);
                true
            }
            Msg::LogoutRespone(_) => {
                self.props.onlogout.emit(());
                true
            }
            Msg::ToLogin => {
                self.route_agent
                    .send(ChangeRoute(AppRoute::LogoutHint.into()));
                true
            }
            Msg::EditCreateId(create_id) => {
                self.state.create_id = create_id;
                false
            }
            Msg::CreateDevice => {
                if self.state.create_id.trim().is_empty() {
                    false
                } else {
                    self.state.err = None;
                    let request = CreateDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        mail: (*self.props.mail).clone(),
                        id: self.state.create_id.trim().to_string(),
                    };
                    crate::create_fetch_task!(
                        self,
                        "/api/create_device",
                        request,
                        CreateDeviceResponse
                    );
                    true
                }
            }
            Msg::CreateDeviceResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.err = None;
                    self.update(Msg::Fetch)
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin)
                } else {
                    self.state.err = Some(fluent!(self.props.lang_id, &response.err));
                    true
                }
            }
            Msg::RemoveDevice(index) => {
                if index >= self.state.devices.len() {
                    false
                } else {
                    self.state.err = None;
                    let request = RemoveDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        mail: (*self.props.mail).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    crate::create_fetch_task!(self, "/api/remove_device", request, RemoveDeviceResponse);
                    true
                }
            }
            Msg::RemoveDeviceResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.err = None;
                    self.update(Msg::Fetch)
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin)
                } else {
                    self.state.err = Some(fluent!(self.props.lang_id, &response.err));
                    true
                }
            }
            Msg::Fetch => {
                self.state.err = None;
                let request = FetchDeviceListRequest {
                    login_token: (*self.props.login_token).clone(),
                    mail: (*self.props.mail).clone(),
                };
                crate::create_fetch_task!(
                    self,
                    "/api/fetch_device_list",
                    request,
                    FetchDeviceListResponse,
                    FetchResponse,
                );
                true
            }
            Msg::FetchResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.err = None;
                    self.state.devices = response.devices;
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin);
                } else {
                    self.state.err = Some(fluent!(self.props.lang_id, &response.err));
                }
                true
            }
            Msg::Modify(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let request = FetchDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    crate::create_fetch_task!(
                        self,
                        "/api/fetch_device",
                        request,
                        FetchDeviceResponse,
                        ModifyResponse,
                    );
                    true
                } else {
                    false
                }
            }
            Msg::ModifyResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.props
                        .onselect
                        .emit((response.id, response.name, response.info));
                    self.route_agent
                        .send(ChangeRoute(AppRoute::ModifyDevice.into()));
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin);
                } else {
                    self.state.err = Some(fluent!(self.props.lang_id, &response.err));
                }
                true
            }
            Msg::Details(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let request = FetchDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    crate::create_fetch_task!(
                        self,
                        "/api/fetch_device",
                        request,
                        FetchDeviceResponse,
                        DetialsResponse,
                    );
                    true
                } else {
                    false
                }
            }
            Msg::DetialsResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.props
                        .onselect
                        .emit((response.id, response.name, response.info));
                    self.route_agent
                        .send(ChangeRoute(AppRoute::DeviceContent.into()));
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin);
                } else {
                    self.state.err = Some(fluent!(self.props.lang_id, &response.err));
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        let create_oninput = self
            .link
            .callback(|e: InputData| Msg::EditCreateId(e.value));
        let create_click = self.link.callback(|_| Msg::CreateDevice);
        let fetch_click = self.link.callback(|_| Msg::Fetch);
        let logout_click = self.link.callback(|_| Msg::Logout);

        html! {
            <div class="container">
                <div class="header">
                    <h2>{ fluent!(self.props.lang_id, "header") }</h2>
                </div>
                <div class="welcome">
                    <p>{ fluent!(self.props.lang_id, "welcome", {
                        "username" => self.props.name.as_str(),
                        "email" => self.props.mail.as_str(),
                    }) }</p>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div class="error-info">
                                <p>{ fluent!(self.props.lang_id, "error-label",
                                    { "details" => err.as_str() }) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="form-item">
                    <MatTextField
                        classes=classes!("form-row-item")
                        outlined=true
                        label=fluent!(self.props.lang_id, "id-label")
                        helper=fluent!(self.props.lang_id, "id-hint")
                        value=self.state.create_id.clone()
                        oninput=create_oninput />
                    <span
                        class="form-row-item"
                        onclick=create_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("form-button")
                            label=fluent!(self.props.lang_id, "button-add")
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                    <span
                        class="form-row-item"
                        onclick=fetch_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("form-button")
                            label=fluent!(self.props.lang_id, "button-fetch")
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                    <span
                        class="form-row-item"
                        onclick=logout_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("logout", "form-button")
                            label=fluent!(self.props.lang_id, "button-logout")
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                </div>
                { self.fetching_progress() }
                <div class="device-list">
                    { self.devices_html() }
                </div>
            </div>
        }
    }
}

impl HomeComponent {
    fn need_to_disable(&self) -> bool {
        self.fetch_task.is_some()
    }

    fn fetching_progress(&self) -> yew::Html {
        if self.fetch_task.is_some() {
            html! {
                <div class="fetching-progress">
                    <MatLinearProgress indeterminate=true />
                </div>
            }
        } else {
            html! {}
        }
    }

    fn devices_html(&self) -> yew::Html {
        html! {
            for self
                .state
                .devices
                .iter()
                .enumerate()
                .map(|(ind, dev)| self.device_html(dev, ind))
        }
    }

    fn device_html(&self, device: &DeviceInfo, index: usize) -> yew::Html {
        let modify_click = self.link.callback(move |_| Msg::Modify(index));
        let detials_click = self.link.callback(move |_| Msg::Details(index));
        let remove_click = self.link.callback(move |_| Msg::RemoveDevice(index));

        html! {
            <CardDiv classes=classes!("device-list-item")>
                <p class="device-name">{ &device.name }</p>
                <p class="device-id">{ &device.id }</p>
                <p class="device-stat">
                    { fluent!(self.props.lang_id, "device-stat", {
                        "total" => device.message_count,
                        "alert" => device.alert_message_count,
                    }) }
                </p>
                <div class="device-buttons">
                    <span onclick=modify_click disabled=self.need_to_disable()>
                        <MatButton
                            label=fluent!(self.props.lang_id, "button-edit")
                            icon=Cow::from("edit")
                            disabled=self.need_to_disable() />
                    </span>
                    <span onclick=detials_click disabled=self.need_to_disable()>
                        <MatButton
                            label=fluent!(self.props.lang_id, "button-details")
                            icon=Cow::from("analytics")
                            disabled=self.need_to_disable() />
                    </span>
                    <span onclick=remove_click disabled=self.need_to_disable()>
                        <MatButton
                            label=fluent!(self.props.lang_id, "button-delete")
                            icon=Cow::from("delete")
                            disabled=self.need_to_disable() />
                    </span>
                </div>
            </CardDiv>
        }
    }
}
