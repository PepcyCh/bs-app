use crate::{fluent, route::AppRoute};
use common::{
    request::LoginRequest,
    response::{ErrorResponse, LoginResponse},
};
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
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
    Bridge, Callback, Component, ComponentLink, InputData, Properties,
};
use yew_material::{text_inputs::TextFieldType, MatButton, MatTextField};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

static_loader! {
    static LOCALES = {
        locales: "./text/login",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct LoginComponent {
    link: ComponentLink<Self>,
    props: Props,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Default)]
struct State {
    mail: String,
    password: String,
    err: Option<String>,
}

pub enum Msg {
    Nop,
    EditMail(String),
    EditPassword(String),
    Login,
    LoginResponse(LoginResponse),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub lang_id: LanguageIdentifier,
    pub onlogin: Callback<(String, String, String)>,
}

impl Component for LoginComponent {
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::EditMail(mail) => {
                self.state.mail = mail;
                false
            }
            Msg::EditPassword(password) => {
                self.state.password = password;
                false
            }
            Msg::Login => {
                self.state.err = None;
                if self.state.mail.is_empty() {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-email-empty"));
                } else if self.state.password.is_empty() {
                    self.state.err = Some(fluent!(self.props.lang_id, "error-password-empty"));
                } else {
                    let hashed_password =
                        format!("{:x}", Sha256::digest(self.state.password.as_bytes()));
                    let request = LoginRequest {
                        mail: self.state.mail.clone(),
                        password: hashed_password,
                    };
                    crate::create_fetch_task!(
                        self,
                        "/login",
                        request,
                        LoginResponse,
                        LoginResponse
                    );
                }
                true
            }
            Msg::LoginResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.route_agent.send(ChangeRoute(AppRoute::Home.into()));
                    self.props
                        .onlogin
                        .emit((response.login_token, response.mail, response.name));
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
        let password_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPassword(e.value));
        let login_click = self.link.callback(|_| Msg::Login);
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
                            value=self.state.mail.clone()
                            oninput=mail_oninput />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            field_type=TextFieldType::Password
                            label=fluent!(self.props.lang_id, "password-label")
                            helper=fluent!(self.props.lang_id, "password-hint")
                            helper_persistent=true
                            value=self.state.password.clone()
                            oninput=password_oninput />
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
                            onclick=login_click
                            class="form-row-item"
                            disabled=self.need_to_disable() >
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "btn-login")
                                disabled=self.need_to_disable()
                                raised=true />
                        </span>
                        <RouterAnchor<AppRoute>
                            route={ AppRoute::Register }
                            classes="form-row-item">
                            <MatButton
                                classes=classes!("form-button")
                                label=fluent!(self.props.lang_id, "btn-register")
                                disabled=self.need_to_disable()
                                raised=true />
                        </RouterAnchor<AppRoute>>
                    </div>
                </div>
            </div>
        }
    }
}

impl LoginComponent {
    fn need_to_disable(&self) -> bool {
        self.fetch_task.is_some()
    }
}
