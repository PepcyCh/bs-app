use crate::{fluent, route::AppRoute};
use common::{
    request::RegisterRequest,
    response::{ErrorResponse, SimpleResponse},
};
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use lazy_static::lazy_static;
use regex::Regex;
use sha2::{Digest, Sha256};
use yew::{
    agent::Bridged,
    classes,
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    web_sys::HtmlInputElement,
    Bridge, Callback, Component, ComponentLink, InputData, NodeRef, Properties,
};
use yew_material::{
    text_inputs::{TextFieldType, ValidityState},
    MatButton, MatTextField,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

static_loader! {
    static LOCALES = {
        locales: "./text/register",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct RegisterComponent {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
    password_ref: NodeRef,
}

#[derive(Default)]
struct State {
    mail: String,
    name: String,
    password: String,
    password_twice: String,
    err: Option<String>,
}

pub enum Msg {
    Nop,
    EditMail(String),
    EditName(String),
    EditPassword(String),
    EditPasswordTwice(String),
    Register,
    RegisterResponse(SimpleResponse),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub lang_id: LanguageIdentifier,
    pub onregister: Callback<()>,
}

lazy_static! {
    static ref MAIL_RE: Regex =
        Regex::new(r"^[0-9a-zA-Z._+-]+@[0-9a-zA-Z-]+\.[0-9a-zA-Z-.]+$").unwrap();
    static ref NAME_RE: Regex = Regex::new(r"^[0-9a-zA-Z_]{4, 32}$").unwrap();
    static ref PASSWORD_RE: Regex = Regex::new(r"^[0-9a-zA-Z_]{6, 32}$").unwrap();
}

impl Component for RegisterComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        Self {
            link,
            props,
            state: State::default(),
            route_agent,
            fetch_task: None,
            password_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::EditMail(mail) => {
                self.state.mail = mail;
                false
            }
            Msg::EditName(name) => {
                self.state.name = name;
                false
            }
            Msg::EditPassword(password) => {
                self.state.password = password;
                false
            }
            Msg::EditPasswordTwice(password) => {
                self.state.password_twice = password;
                false
            }
            Msg::Register => {
                self.state.err = None;
                if !MAIL_RE.is_match(&self.state.mail) {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-email"));
                } else if !NAME_RE.is_match(&self.state.name) {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-username"));
                } else if !PASSWORD_RE.is_match(&self.state.password) {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-password"));
                } else if self.state.password != self.state.password_twice {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-password2"));
                } else {
                    let hashed_password =
                        format!("{:x}", Sha256::digest(self.state.password.as_bytes()));
                    let request = RegisterRequest {
                        mail: self.state.mail.clone(),
                        name: self.state.name.clone(),
                        password: hashed_password,
                    };
                    crate::create_fetch_task!(self, "/register", request, RegisterResponse);
                }
                true
            }
            Msg::RegisterResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                    self.props.onregister.emit(());
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
        let mail_oninput = self.link.callback(|e: InputData| Msg::EditMail(e.value));
        let name_oninput = self.link.callback(|e: InputData| Msg::EditName(e.value));
        let password_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPassword(e.value));
        let password2_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPasswordTwice(e.value));
        let register_click = self.link.callback(|_| Msg::Register);

        let mail_validate = MatTextField::validity_transform(|str, _| {
            if !MAIL_RE.is_match(&str) {
                let mut state = ValidityState::new();
                state.set_valid(false).set_bad_input(true);
                state
            } else {
                ValidityState::new()
            }
        });

        let name_validate = MatTextField::validity_transform(|str, _| {
            if !NAME_RE.is_match(&str) {
                let mut state = ValidityState::new();
                state.set_valid(false).set_bad_input(true);
                state
            } else {
                ValidityState::new()
            }
        });

        let password_validate = MatTextField::validity_transform(|str, _| {
            if !PASSWORD_RE.is_match(&str) {
                let mut state = ValidityState::new();
                state.set_valid(false).set_bad_input(true);
                state
            } else {
                ValidityState::new()
            }
        });

        let password_ref = self.password_ref.clone();
        let password2_validate = MatTextField::validity_transform(move |str, _| {
            if let Some(password_ele) = password_ref.cast::<HtmlInputElement>() {
                let password = password_ele.value();
                if str != password {
                    let mut state = ValidityState::new();
                    state.set_valid(false).set_bad_input(true);
                    state
                } else {
                    ValidityState::new()
                }
            } else {
                ValidityState::new()
            }
        });

        html! {
            <div class="container">
                <div class="form">
                    <div class="header">
                        <h2>{ fluent!(self.props.lang_id, "header") }</h2>
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            label=fluent!(self.props.lang_id, "email-label")
                            helper=fluent!(self.props.lang_id, "email-hint")
                            helper_persistent=true
                            validity_transform=mail_validate
                            validation_message=fluent!(self.props.lang_id, "email-inv")
                            value=self.state.mail.clone()
                            oninput=mail_oninput />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            label=fluent!(self.props.lang_id, "username-label")
                            helper=fluent!(self.props.lang_id, "username-hint")
                            helper_persistent=true
                            validity_transform=name_validate
                            validation_message=fluent!(self.props.lang_id, "username-inv")
                            value=self.state.name.clone()
                            oninput=name_oninput />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            field_type=TextFieldType::Password
                            label=fluent!(self.props.lang_id, "password-label")
                            helper=fluent!(self.props.lang_id, "password-hint")
                            helper_persistent=true
                            validity_transform=password_validate
                            validation_message=fluent!(self.props.lang_id, "password-inv")
                            value=self.state.password.clone()
                            oninput=password_oninput
                            ref=self.password_ref.clone() />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            field_type=TextFieldType::Password
                            label=fluent!(self.props.lang_id, "password2-label")
                            helper=fluent!(self.props.lang_id, "password2-hint")
                            helper_persistent=true
                            validity_transform=password2_validate
                            validation_message=fluent!(self.props.lang_id, "password2-inv")
                            value=self.state.password_twice.clone()
                            oninput=password2_oninput />
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
                        <span
                            onclick=register_click
                            class="form-row-item"
                            disabled=self.need_to_disable() >
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "button-register")
                                disabled=self.need_to_disable()
                                raised=true />
                        </span>
                        <RouterAnchor<AppRoute>
                            route={ AppRoute::Login }
                            classes="form-row-item">
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "button-login")
                                disabled=self.need_to_disable()
                                raised=true />
                        </RouterAnchor<AppRoute>>
                    </div>
                </div>
            </div>
        }
    }
}

impl RegisterComponent {
    fn need_to_disable(&self) -> bool {
        self.fetch_task.is_some()
    }
}
