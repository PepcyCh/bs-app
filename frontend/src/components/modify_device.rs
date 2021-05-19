use std::rc::Rc;

use common::{
    request::ModifyDeviceRequest,
    response::{ErrorResponse, SimpleResponse},
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

pub struct ModifyDevice {
    link: ComponentLink<Self>,
    state: State,
    fetch_task: Option<FetchTask>,
}

struct State {
    id: String,
    name: String,
    info: String,
    err: Option<String>,
    success_hint: Option<String>,
}

pub enum Msg {
    EditName(String),
    EditInfo(String),
    Save,
    SaveResponse(SimpleResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub info: Rc<String>,
}

impl Component for ModifyDevice {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            id: (*props.id).clone(),
            name: (*props.name).clone(),
            info: (*props.info).clone(),
            err: None,
            success_hint: None,
        };
        Self {
            link,
            state,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::EditName(name) => {
                self.state.name = name;
                false
            }
            Msg::EditInfo(info) => {
                self.state.info = info;
                false
            }
            Msg::Save => {
                self.state.err = None;
                self.state.success_hint = None;
                let modify_info = ModifyDeviceRequest {
                    id: self.state.id.clone(),
                    name: self.state.name.clone(),
                    info: self.state.info.clone(),
                };
                let body = serde_json::to_value(&modify_info).unwrap();
                let request = Request::post("/modify_device")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to construct modify request");
                let callback = self.link.callback(
                    |response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                        let Json(data) = response.into_body();
                        if let Ok(result) = data {
                            Msg::SaveResponse(result)
                        } else {
                            Msg::SaveResponse(SimpleResponse::err("Unknown error"))
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::SaveResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.success_hint =
                        Some("Device info is modified successfully".to_string());
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
        let name_oninput = self.link.callback(|e: InputData| Msg::EditName(e.value));
        let info_oninput = self.link.callback(|e: InputData| Msg::EditInfo(e.value));
        let save_click = self.link.callback(|_| Msg::Save);

        html! {
            <div>
                <h1>{ "Modify Device Info" }</h1>
                {
                    if let Some(hint) = &self.state.success_hint {
                        html! {
                            <div>
                                <p>{ hint }</p>
                            </div>
                        }
                    } else if let Some(err) = &self.state.err {
                        html! {
                            <div>
                                <p>{ format!("Failed to modify: {}", err) }</p>
                            </div>
                        }
                    } else{
                        html! {}
                    }
                }
                <div>
                    { format!("ID: {}", &self.state.id) }
                    { "Name: " }
                    <input
                        value=self.state.name.clone()
                        oninput=name_oninput />
                    { "Infomation:" }
                    <input
                        value=self.state.info.clone()
                        oninput=info_oninput />
                    <button
                        onclick=save_click
                        disabled=self.fetch_task.is_some() >
                        { "Save" }
                    </button>
                </div>
                <RouterAnchor<AppRoute> route={ AppRoute::Home }>
                    { "Go back to Home" }
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
