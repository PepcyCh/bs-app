use std::rc::Rc;

use common::{
    request::FetchMessageListRequest,
    response::{ErrorResponse, FetchMessageListResponse, MessageInfo},
};
use yew::{
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, InputData, Properties,
};
use yew_router::prelude::*;

use crate::route::AppRoute;

pub struct DeviceContent {
    link: ComponentLink<Self>,
    state: State,
    fetch_task: Option<FetchTask>,
}

struct State {
    id: String,
    name: String,
    info: String,
    start_timestamp_str: String,
    end_timestamp_str: String,
    messages: Vec<MessageInfo>,
    err: Option<String>,
}

pub enum Msg {
    EditStartTime(String),
    EditEndTime(String),
    Fetch,
    FetchResponse(FetchMessageListResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub info: Rc<String>,
}

impl Component for DeviceContent {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            id: (*props.id).clone(),
            name: (*props.name).clone(),
            info: (*props.info).clone(),
            start_timestamp_str: "".to_string(),
            end_timestamp_str: "".to_string(),
            messages: vec![],
            err: None,
        };
        Self {
            link,
            state,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::EditStartTime(start_timestamp_str) => {
                self.state.start_timestamp_str = start_timestamp_str;
                false
            }
            Msg::EditEndTime(end_timestamp_str) => {
                self.state.end_timestamp_str = end_timestamp_str;
                false
            }
            Msg::Fetch => {
                self.state.err = None;
                let start_timestamp = self
                    .state
                    .start_timestamp_str
                    .parse::<u64>()
                    .or::<()>(Ok(0))
                    .unwrap();
                let end_timestamp = self
                    .state
                    .end_timestamp_str
                    .parse::<u64>()
                    .or::<()>(Ok(u64::MAX))
                    .unwrap();
                let fetch_info = FetchMessageListRequest {
                    id: self.state.id.clone(),
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
                } else {
                    self.state.err = Some(response.err);
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let start_time_oninput = self
            .link
            .callback(|e: InputData| Msg::EditStartTime(e.value));
        let end_time_oninput = self.link.callback(|e: InputData| Msg::EditEndTime(e.value));
        let fetch_click = self.link.callback(|_| Msg::Fetch);

        html! {
            <div>
                <h1>{ &self.state.name }</h1>
                <div>
                    <p>{ format!("ID: {}", &self.state.id) }</p>
                    <p>{ &self.state.info }</p>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div>
                                <p>{ format!("Failed to fetch data: {}", err) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <RouterAnchor<AppRoute> route={ AppRoute::Home }>
                    { "Go back to Home" }
                </RouterAnchor<AppRoute>>
                <RouterAnchor<AppRoute> route={ AppRoute::ModifyDevice }>
                    { "Modify" }
                </RouterAnchor<AppRoute>>
                <div>
                    <input
                        placeholder="start time"
                        type="number"
                        value=&self.state.start_timestamp_str
                        oninput=start_time_oninput />
                    <input
                        placeholder="end time"
                        type="number"
                        value=&self.state.end_timestamp_str
                        oninput=end_time_oninput />
                    <button
                        onclick=fetch_click
                        disabled=self.fetch_task.is_some() >
                        { "Fetch Messages" }
                    </button>
                </div>
                <ul>{ self.messages_html() }</ul>
            </div>
        }
    }
}

impl DeviceContent {
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
        html! {
            <li>
                <div>
                {
                    if msg.alert {
                        html! {
                            <p>{ "ALERT" }</p>
                        }
                    } else {
                        html! {
                            <p>{ "MESSAGE" } </p>
                        }
                    }
                }
                </div>
                {
                    format!("value: {}, location: ({}, {}), time: {}",
                        msg.value, msg.lng, msg.lat, msg.timestamp)
                }
            </li>
        }
    }
}
