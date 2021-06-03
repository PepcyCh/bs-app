use std::rc::Rc;

use chrono::{DateTime, TimeZone, Utc};
use common::{
    request::{FetchDeviceProfileRequest, FetchMessageListRequest},
    response::{ErrorResponse, FetchDeviceProfileResponse, FetchMessageListResponse, MessageInfo},
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
    Bridge, Component, ComponentLink, InputData, Properties,
};
use yew_material::{MatButton, MatLinearProgress};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::{
    route::AppRoute,
    utils::{
        card_div::CardDiv,
        line_chart::{LineChart, LineChartData},
        map::{Map, MapPointData},
        paged_list::PagedList,
    },
};

pub struct DeviceContent {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    start_timestamp_str: String,
    end_timestamp_str: String,
    message_count: u32,
    alert_message_count: u32,
    first_index: usize,
    limit: usize,
    messages: Vec<MessageInfo>,
    err: Option<String>,
}

pub enum Msg {
    Nop,
    ToLogin,
    EditStartTime(String),
    EditEndTime(String),
    FetchProfile,
    FetchProfileResponse(FetchDeviceProfileResponse),
    Fetch,
    FetchResponse(FetchMessageListResponse),
    ChangePage(usize, usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub login_token: Rc<String>,
    pub mail: Rc<String>,
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub info: Rc<String>,
}

impl Component for DeviceContent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let state = State {
            start_timestamp_str: "".to_string(),
            end_timestamp_str: "".to_string(),
            limit: 20,
            ..Default::default()
        };
        let mut component = Self {
            link,
            props,
            state,
            route_agent,
            fetch_task: None,
        };
        if component.props.login_token.is_empty() {
            component.update(Msg::ToLogin);
        } else {
            component.update(Msg::FetchProfile);
        }
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::ToLogin => {
                self.route_agent
                    .send(ChangeRoute(AppRoute::LogoutHint.into()));
                true
            }
            Msg::EditStartTime(start_timestamp_str) => {
                self.state.start_timestamp_str = start_timestamp_str;
                false
            }
            Msg::EditEndTime(end_timestamp_str) => {
                self.state.end_timestamp_str = end_timestamp_str;
                false
            }
            Msg::FetchProfile => {
                self.state.err = None;
                let request = FetchDeviceProfileRequest {
                    login_token: (*self.props.login_token).clone(),
                    id: (*self.props.id).clone(),
                };
                crate::create_fetch_task!(
                    self,
                    "/fetch_device_profile",
                    request,
                    FetchDeviceProfileResponse,
                    FetchProfileResponse,
                );
                true
            }
            Msg::FetchProfileResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.message_count = response.message_count;
                    self.state.alert_message_count = response.alert_message_count;
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin);
                } else {
                    self.state.err = Some(response.err);
                }
                self.update(Msg::Fetch)
            }
            Msg::Fetch => {
                self.state.err = None;
                let start_timestamp = if let Ok(datetime) =
                    DateTime::parse_from_str(&self.state.start_timestamp_str, "%Y-%m-%dT%H:%M")
                {
                    datetime.timestamp()
                } else {
                    0
                };
                let end_timestamp = if let Ok(datetime) =
                    DateTime::parse_from_str(&self.state.end_timestamp_str, "%Y-%m-%dT%H:%M")
                {
                    datetime.timestamp()
                } else {
                    std::i64::MAX
                };
                let request = FetchMessageListRequest {
                    login_token: (*self.props.login_token).clone(),
                    id: (*self.props.id).clone(),
                    start_timestamp,
                    end_timestamp,
                    first_index: self.state.first_index,
                    limit: self.state.limit,
                };
                crate::create_fetch_task!(
                    self,
                    "/fetch_message_list",
                    request,
                    FetchMessageListResponse,
                    FetchResponse,
                );
                true
            }
            Msg::FetchResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.messages = response.messages;
                } else if response.err == "Login has expired" {
                    self.update(Msg::ToLogin);
                } else {
                    self.state.err = Some(response.err);
                }
                true
            }
            Msg::ChangePage(first_index, limit) => {
                self.state.first_index = first_index;
                self.state.limit = limit;
                self.update(Msg::Fetch)
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
        let start_time_oninput = self
            .link
            .callback(|e: InputData| Msg::EditStartTime(e.value));
        let end_time_oninput = self.link.callback(|e: InputData| Msg::EditEndTime(e.value));
        let fetch_click = self.link.callback(|_| Msg::Fetch);
        let list_on_page_changed = self
            .link
            .callback(|data: (usize, usize)| Msg::ChangePage(data.0, data.1));

        html! {
            <div class="container">
                <div class="header">
                    <h2>{ &self.props.name }</h2>
                </div>
                <div class="device-info">
                    <p class="device-id">{ format!("ID: {}", &self.props.id) }</p>
                    <p class="info">{ &self.props.info }</p>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div class="error-info">
                                <p>{ format!("Failed to fetch data: {}", err) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="form-item">
                    <RouterAnchor<AppRoute>
                        route={ AppRoute::Home }
                        classes="form-row-item" >
                        <MatButton
                            classes=classes!("form-button")
                            label="Back"
                            raised=true
                            disabled=self.need_to_disable() />
                    </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute>
                        route={ AppRoute::ModifyDevice }
                        classes="form-row-item" >
                        <MatButton
                            classes=classes!("form-button")
                            label="Modify"
                            raised=true
                            disabled=self.need_to_disable() />
                    </RouterAnchor<AppRoute>>
                    // TODO - custom datetme input ?
                    <input
                        class="form-row-item"
                        type="datetime-local"
                        value=self.state.start_timestamp_str.clone()
                        oninput=start_time_oninput />
                    <input
                        class="form-row-item"
                        type="datetime-local"
                        value=self.state.end_timestamp_str.clone()
                        oninput=end_time_oninput />
                    <span
                        class="form-row-item"
                        onclick=fetch_click
                        disabled=self.need_to_disable() >
                        <MatButton
                            classes=classes!("form-button")
                            label="Fetch Messages"
                            raised=true
                            disabled=self.need_to_disable() />
                    </span>
                </div>
                { self.fetching_progress() }
                <p>
                    { format!("{} messages in total, {} are alert",
                        self.state.message_count, self.state.alert_message_count) }
                </p>
                <div class="device-map">
                    { self.message_map() }
                </div>
                <div class="device-charts">
                    { self.message_line_chart() }
                </div>
                <PagedList
                    page_size=20
                    items_count=self.state.message_count as usize
                    disabled=self.need_to_disable()
                    on_page_changed=list_on_page_changed >
                    { self.messages_html() }
                </PagedList>
            </div>
        }
    }
}

impl DeviceContent {
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

    fn messages_html(&self) -> yew::Html {
        html! {
            for self
                .state
                .messages
                .iter()
                .map(|msg| self.message_html(msg))
        }
    }

    fn message_html(&self, msg: &MessageInfo) -> yew::Html {
        let time = Utc.timestamp(msg.timestamp / 1000, 0);
        html! {
            <CardDiv>
                {
                    if msg.alert {
                        html! {
                            <div class="material-icons message-alert">
                                { "warning" }
                            </div>
                        }
                    } else {
                        html! {
                            <div class="material-icons message-normal">
                                { "check_circle" }
                            </div>
                        }
                    }
                }
                <p>{ format!("value: {}", msg.value) }</p>
                <p>{ format!("position: ({}, {})", msg.lng, msg.lat) }</p>
                <p>{ format!("time: {}", time) }</p>
            </CardDiv>
        }
    }

    fn message_line_chart(&self) -> yew::Html {
        let for_upper = self.state.messages.len().min(20);
        let mut data = Vec::with_capacity(for_upper);
        for i in 0..for_upper {
            let item = LineChartData {
                x: self.state.messages[i].timestamp as f64,
                y: self.state.messages[i].value as f64,
            };
            data.push(item);
        }
        let data = Rc::new(data);
        html! {
            <div class="device-charts-item">
                <LineChart data=data.clone() height=400 />
            </div>
        }
    }

    fn message_map(&self) -> yew::Html {
        let data: Vec<_> = self
            .state
            .messages
            .iter()
            .map(|msg| MapPointData {
                x: msg.lng as f64,
                y: msg.lat as f64,
                value: msg.value as f64,
            })
            .collect();
        let data = Rc::new(data);
        html! {
            <div class="device-map-item">
                <Map data=data.clone() />
            </div>
        }
    }
}
