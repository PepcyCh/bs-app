use std::rc::Rc;

use chrono::{DateTime, TimeZone, Utc};
use common::{
    request::FetchMessageListRequest,
    response::{ErrorResponse, FetchMessageListResponse, MessageInfo},
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

use crate::route::AppRoute;

pub struct DeviceContent {
    link: ComponentLink<Self>,
    props: Prop,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
}

struct State {
    start_timestamp_str: String,
    end_timestamp_str: String,
    messages: Vec<MessageInfo>,
    err: Option<String>,
}

pub enum Msg {
    Nop,
    ToLogin,
    EditStartTime(String),
    EditEndTime(String),
    Fetch,
    FetchResponse(FetchMessageListResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub login_token: Rc<String>,
    pub mail: Rc<String>,
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub info: Rc<String>,
}

impl Component for DeviceContent {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let state = State {
            start_timestamp_str: "".to_string(),
            end_timestamp_str: "".to_string(),
            messages: vec![],
            err: None,
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
            component.update(Msg::Fetch);
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
                // yew::services::ConsoleService::log(&format!("start time = {}", &start_timestamp_str));
                self.state.start_timestamp_str = start_timestamp_str;
                false
            }
            Msg::EditEndTime(end_timestamp_str) => {
                // yew::services::ConsoleService::log(&format!("end time = {}", &end_timestamp_str));
                self.state.end_timestamp_str = end_timestamp_str;
                false
            }
            Msg::Fetch => {
                self.state.err = None;
                // yew::services::ConsoleService::log(&format!("start time = {}, end time = {}",
                //     &self.state.start_timestamp_str, &self.state.end_timestamp_str));
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
                let fetch_info = FetchMessageListRequest {
                    login_token: (*self.props.login_token).clone(),
                    id: (*self.props.id).clone(),
                    start_timestamp,
                    end_timestamp,
                };
                let body = serde_json::to_value(&fetch_info).unwrap();
                let request = Request::post("/fetch_message_list")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to construct fetch message list request");
                let callback = self.link.callback(
                    |response: Response<Json<anyhow::Result<FetchMessageListResponse>>>| {
                        let Json(data) = response.into_body();
                        if let Ok(result) = data {
                            Msg::FetchResponse(result)
                        } else {
                            Msg::FetchResponse(FetchMessageListResponse::err("Unknown error"))
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
                    self.state.messages = response.messages;
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
        let start_time_oninput = self
            .link
            .callback(|e: InputData| Msg::EditStartTime(e.value));
        let end_time_oninput = self.link.callback(|e: InputData| Msg::EditEndTime(e.value));
        let fetch_click = self.link.callback(|_| Msg::Fetch);

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
                    // TODO - custom datetme input
                    // <MatTextField
                    //     classes=classes!("form-row-item")
                    //     outlined=true
                    //     label="Start Time"
                    //     field_type=TextFieldType::DatetimeLocal
                    //     value=self.state.start_timestamp_str.clone()
                    //     oninput=start_time_oninput />
                    <input
                        class="form-row-item"
                        type="datetime-local"
                        value=self.state.start_timestamp_str.clone()
                        oninput=start_time_oninput />
                    // <MatTextField
                    //     classes=classes!("form-row-item")
                    //     outlined=true
                    //     label="End Time"
                    //     field_type=TextFieldType::DatetimeLocal
                    //     value=self.state.end_timestamp_str.clone()
                    //     oninput=end_time_oninput />
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
                // TODO - list page
                <div class="message-list">
                    { self.messages_html() }
                </div>
                // TODO - message graph
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
            <div class="message-list-item">
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
            </div>
        }
    }
}
