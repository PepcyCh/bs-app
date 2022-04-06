use crate::{fluent, route::AppRoute};
use common::{
    request::ModifyDeviceRequest,
    response::{ErrorResponse, SimpleResponse},
};
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use lazy_static::lazy_static;
use regex::Regex;
use std::rc::Rc;
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
use yew_material::{text_inputs::ValidityState, MatButton, MatTextArea, MatTextField};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

static_loader! {
    static LOCALES = {
        locales: "./text/modify_device",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct ModifyDevice {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
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
    Nop,
    ToLogin,
    EditName(String),
    EditInfo(String),
    Save,
    SaveResponse(SimpleResponse),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub lang_id: LanguageIdentifier,
    pub login_token: Rc<String>,
    pub mail: Rc<String>,
    pub id: Rc<String>,
    pub name: Rc<String>,
    pub info: Rc<String>,
}

lazy_static! {
    static ref NAME_RE: Regex = Regex::new(r"^[0-9a-zA-Z_\s]{4, 32}$").unwrap();
}

impl Component for ModifyDevice {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let state = State {
            id: (*props.id).clone(),
            name: (*props.name).clone(),
            info: (*props.info).clone(),
            err: None,
            success_hint: None,
        };
        let mut component = Self {
            props,
            link,
            state,
            route_agent,
            fetch_task: None,
        };
        if component.props.login_token.is_empty() {
            component.update(Msg::ToLogin);
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
            Msg::EditName(name) => {
                self.state.name = name;
                false
            }
            Msg::EditInfo(info) => {
                self.state.info = info;
                false
            }
            Msg::Save => {
                if !NAME_RE.is_match(&self.state.name) {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-name"));
                    true
                } else {
                    self.state.err = None;
                    self.state.success_hint = None;
                    let request = ModifyDeviceRequest {
                        login_token: (*self.props.login_token).clone(),
                        id: self.state.id.clone(),
                        name: self.state.name.clone(),
                        info: self.state.info.clone(),
                    };
                    crate::create_fetch_task!(self, "/api/modify_device", request, SaveResponse);
                    true
                }
            }
            Msg::SaveResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.state.success_hint = Some(fluent!(self.props.lang_id, "success-info"));
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
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        let name_oninput = self.link.callback(|e: InputData| Msg::EditName(e.value));
        let info_oninput = self.link.callback(|e: InputData| Msg::EditInfo(e.value));
        let save_click = self.link.callback(|_| Msg::Save);

        let name_validate = MatTextField::validity_transform(|str, _| {
            if !NAME_RE.is_match(&str) {
                let mut state = ValidityState::new();
                state.set_valid(false).set_bad_input(true);
                state
            } else {
                ValidityState::new()
            }
        });

        html! {
            <div class="container">
                <div class="header">
                    <h2>{ fluent!(self.props.lang_id, "header") }</h2>
                </div>
                <div class="form">
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            label=fluent!(self.props.lang_id, "id-label")
                            value=self.state.id.clone()
                            disabled=true />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            label=fluent!(self.props.lang_id, "name-label")
                            helper=fluent!(self.props.lang_id, "name-hint")
                            helper_persistent=true
                            validity_transform=name_validate
                            validation_message=fluent!(self.props.lang_id, "name-inv")
                            value=self.state.name.clone()
                            oninput=name_oninput />
                    </div>
                    <div class="form-item">
                        <MatTextArea
                            classes=classes!("form-input")
                            outlined=true
                            label=fluent!(self.props.lang_id, "desc-label")
                            helper=fluent!(self.props.lang_id, "name-hint")
                            helper_persistent=true
                            max_length=256
                            value=self.state.info.clone()
                            oninput=info_oninput />
                    </div>
                    {
                        if let Some(hint) = &self.state.success_hint {
                            html! {
                                <div class="hint-info">
                                    <p>{ hint }</p>
                                </div>
                            }
                        } else if let Some(err) = &self.state.err {
                            html! {
                                <div class="error-info">
                                    <p>{ fluent!(self.props.lang_id, "error-label",
                                        { "details" => err.as_str() } ) }</p>
                                </div>
                            }
                        } else{
                            html! {}
                        }
                    }
                    <div class="form-item">
                        <span
                            onclick=save_click
                            class="form-row-item"
                            disabled=self.need_to_disable() >
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "button-save")
                                disabled=self.need_to_disable()
                                raised=true />
                        </span>
                        <RouterAnchor<AppRoute>
                            route={ AppRoute::Home }
                            classes="form-row-item">
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "button-home")
                                disabled=self.need_to_disable()
                                raised=true />
                        </RouterAnchor<AppRoute>>
                    </div>
                </div>
            </div>
        }
    }
}

impl ModifyDevice {
    fn need_to_disable(&self) -> bool {
        self.fetch_task.is_some()
    }
}
