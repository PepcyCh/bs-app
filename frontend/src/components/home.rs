use std::rc::Rc;

use common::{request::{CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceRequest}, response::{DeviceInfo, ErrorResponse, FetchDeviceListResponse, FetchDeviceResponse, SimpleResponse}};
use yew::{agent::Bridged, Bridge, Callback, Component, ComponentLink, InputData, Properties, format::Json, html, services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    }};
use yew_router::{prelude::RouteAgent, agent::RouteRequest::ChangeRoute};

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
    EditCreateId(String),
    CreateDevice,
    CreateDeviceResponse(SimpleResponse),
    Fetch,
    FetchResponse(FetchDeviceListResponse),
    Modify(usize),
    ModifyResponse(FetchDeviceResponse),
    Details(usize),
    DetialsResponse(FetchDeviceResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub mail: Rc<String>,
    pub name: Rc<String>,
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
        component.update(Msg::Fetch);
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::EditCreateId(create_id) => {
                self.state.create_id = create_id;
                false
            }
            Msg::CreateDevice => {
                self.state.err = None;
                let create_info = CreateDeviceRequest {
                    mail: (*self.props.mail).clone(),
                    id: self.state.create_id.clone(),
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
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::CreateDeviceResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.err = None;
                    self.update(Msg::Fetch)
                } else {
                    self.state.err = Some(response.err);
                    true
                }
            }
            Msg::Fetch => {
                self.state.err = None;
                let fetch_info = FetchDeviceListRequest {
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
                } else {
                    self.state.err = Some(response.err);
                }
                true
            }
            Msg::Modify(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let fetch_info = FetchDeviceRequest {
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
                    let task = FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
                    true
                } else {
                    false
                }
            }
            Msg::ModifyResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.props.onselect.emit((response.id, response.name, response.info));
                    self.route_agent.send(ChangeRoute(AppRoute::ModifyDevice.into()));
                } else {
                    self.state.err = Some(response.err);
                }
                true
            }
            Msg::Details(index) => {
                if index < self.state.devices.len() {
                    self.state.err = None;
                    let fetch_info = FetchDeviceRequest {
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
                    let task = FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
                    true
                } else {
                    false
                }
            }
            Msg::DetialsResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.props.onselect.emit((response.id, response.name, response.info));
                    self.route_agent.send(ChangeRoute(AppRoute::DeviceContent.into()));
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

        html! {
            <div>
                <h1>{ "Home" }</h1>
                <div>
                    <p>{ format!("Welcome, {}({})!", &self.props.name, &self.props.mail) }</p>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div>
                                <p>{ format!("Error: {}", err) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div>
                    { "Add device: " }
                    <input
                        value=&self.state.create_id,
                        oninput=create_oninput />
                    <button
                        onclick=create_click
                        disabled=self.fetch_task.is_some() >
                        { "Add" }
                    </button>
                    <button
                        onclick=fetch_click
                        disabled=self.fetch_task.is_some() >
                        { "Fetch Devices" }
                    </button>
                </div>
                <ul>{ self.devices_html() }</ul>
            </div>
        }
    }
}

impl HomeComponent {
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

        html! {
            <li>
                {
                    format!("ID: {}, name: {}, message count: {}, alert message count: {}",
                        &device.id, &device.name, device.message_count, device.alert_message_count)
                }
                <div>
                    <button
                        onclick=modify_click
                        disabled=self.fetch_task.is_some()>
                        { "Modify" }
                    </button>
                    <button
                        onclick=detials_click
                        disabled=self.fetch_task.is_some()>
                        { "Details" }
                    </button>
                </div>
            </li>
        }
    }
}
