use std::{borrow::Cow, rc::Rc};

use common::{
    request::{
        CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceRequest, RemoveDeviceRequest,
    },
    response::{
        DeviceInfo, ErrorResponse, FetchDeviceListResponse, FetchDeviceResponse, SimpleResponse,
    },
};
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
use yew_material::{MatButton, MatLinearProgress, MatList, MatListItem, MatTextField};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};

use crate::route::AppRoute;

pub struct HomeComponent {
    link: ComponentLink<Self>,
    props: Prop,
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
pub struct Prop {
    pub login_token: Rc<String>,
    pub mail: Rc<String>,
    pub name: Rc<String>,
    pub onlogout: Callback<()>,
    pub onselect: Callback<(String, String, String)>,
}

impl Component for HomeComponent {
    type Message = Msg;
    type Properties = Prop;

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
                let body = serde_json::to_value(login_token).unwrap();
                let request = Request::post("/logout")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to construct logout request");
                let callback = self.link.callback(
                    |response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                        let Json(data) = response.into_body();
                        if let Ok(result) = data {
                            Msg::LogoutRespone(result)
                        } else {
                            Msg::LogoutRespone(SimpleResponse::err("Unknown error"))
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
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
                    let create_info = CreateDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        mail: (*self.props.mail).clone(),
                        id: self.state.create_id.trim().to_string(),
                    };
                    let body = serde_json::to_value(&create_info).unwrap();
                    let request = Request::post("/create_device")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct create device request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(result) = data {
                                Msg::CreateDeviceResponse(result)
                            } else {
                                Msg::CreateDeviceResponse(SimpleResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
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
                    self.state.err = Some(response.err);
                    true
                }
            }
            Msg::RemoveDevice(index) => {
                if index >= self.state.devices.len() {
                    false
                } else {
                    self.state.err = None;
                    let create_info = RemoveDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        mail: (*self.props.mail).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    let body = serde_json::to_value(&create_info).unwrap();
                    let request = Request::post("/remove_device")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct remove device request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(result) = data {
                                Msg::RemoveDeviceResponse(result)
                            } else {
                                Msg::RemoveDeviceResponse(SimpleResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
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
                    self.state.err = Some(response.err);
                    true
                }
            }
            Msg::Fetch => {
                self.state.err = None;
                let fetch_info = FetchDeviceListRequest {
                    login_token: (*self.props.login_token).clone(),
                    mail: (*self.props.mail).clone(),
                };
                let body = serde_json::to_value(&fetch_info).unwrap();
                let request = Request::post("/fetch_device_list")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to construct fetch device list request");
                let callback = self.link.callback(
                    |response: Response<Json<anyhow::Result<FetchDeviceListResponse>>>| {
                        let Json(data) = response.into_body();
                        if let Ok(result) = data {
                            Msg::FetchResponse(result)
                        } else {
                            Msg::FetchResponse(FetchDeviceListResponse::err("Unknown error"))
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
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
                    self.state.err = Some(response.err);
                }
                true
            }
            Msg::Modify(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let fetch_info = FetchDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    let body = serde_json::to_value(&fetch_info).unwrap();
                    let request = Request::post("/fetch_device")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct fetch device request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<FetchDeviceResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(result) = data {
                                Msg::ModifyResponse(result)
                            } else {
                                Msg::ModifyResponse(FetchDeviceResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
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
                    self.state.err = Some(response.err);
                }
                true
            }
            Msg::Details(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let fetch_info = FetchDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        id: self.state.devices[index].id.clone(),
                    };
                    let body = serde_json::to_value(&fetch_info).unwrap();
                    let request = Request::post("/fetch_device")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct fetch device request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<FetchDeviceResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(result) = data {
                                Msg::DetialsResponse(result)
                            } else {
                                Msg::DetialsResponse(FetchDeviceResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
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
                    self.state.err = Some(response.err);
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
                    <h2>{ "Home" }</h2>
                </div>
                <div class="welcome">
                    <p>{ format!("Welcome, {}({})!", &self.props.name, &self.props.mail) }</p>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div class="error-info">
                                <p>{ format!("Error: {}", err) }</p>
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
                        label="Device ID"
                        helper="device ID to be added"
                        value=self.state.create_id.clone()
                        oninput=create_oninput />
                    <span
                        class="form-row-item"
                        onclick=create_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("form-button")
                            label="Add Device"
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                    <span
                        class="form-row-item"
                        onclick=fetch_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("form-button")
                            label="Fecth Devices"
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                    <span
                        class="form-row-item"
                        onclick=logout_click
                        disabled=self.need_to_disable()>
                        <MatButton
                            classes=classes!("logout", "form-button")
                            label="Logout"
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                </div>
                { self.fetching_progress() }
                // TODO - list page
                <div class="device-list">
                    <MatList>
                        { self.devices_html() }
                    </MatList>
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
            <MatListItem>
                <div class="device-list-item">
                    <span class="device-name">{ &device.name }</span>
                    <span class="device-id">{ &device.id }</span>
                    <span class="device-stat">
                        { format!("{} messages ({} are alert)",
                            device.message_count, device.alert_message_count) }
                    </span>
                    <span onclick=modify_click disabled=self.need_to_disable()>
                        <MatButton
                            label=""
                            icon=Cow::from("edit")
                            disabled=self.need_to_disable() />
                    </span>
                    <span onclick=detials_click disabled=self.need_to_disable()>
                        <MatButton
                            label=""
                            icon=Cow::from("analytics")
                            disabled=self.need_to_disable() />
                    </span>
                    <span onclick=remove_click disabled=self.need_to_disable()>
                        <MatButton
                            label=""
                            icon=Cow::from("delete")
                            disabled=self.need_to_disable() />
                    </span>
                </div>
            </MatListItem>
        }
    }
}
