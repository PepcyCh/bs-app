use crate::route::AppRoute;
use common::{
    request::LoginRequest,
    response::{ErrorResponse, LoginResponse},
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
use yew_material::{text_inputs::TextFieldType, MatButton, MatTextField};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

pub struct LoginComponent {
    link: ComponentLink<Self>,
    props: Prop,
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
pub struct Prop {
    pub onlogin: Callback<(String, String)>,
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = Prop;

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
                if self.state.mail.is_empty() {
                    self.state.err = Some("E-mail address is empty".to_string());
                } else if self.state.password.is_empty() {
                    self.state.err = Some("Password is empty".to_string());
                } else {
                    self.state.err = None;
                    let login_info = LoginRequest {
                        mail: self.state.mail.clone(),
                        password: self.state.password.clone(),
                    };
                    let body = serde_json::to_value(&login_info).unwrap();
                    let request = Request::post("/login")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct login request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<LoginResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(result) = data {
                                Msg::LoginResponse(result)
                            } else {
                                Msg::LoginResponse(LoginResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
                }
                true
            }
            Msg::LoginResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.route_agent.send(ChangeRoute(AppRoute::Home.into()));
                    self.props.onlogin.emit((response.mail, response.name));
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
        let mail_oninput = self.link.callback(|e: InputData| Msg::EditMail(e.value));
        let password_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPassword(e.value));
        let login_click = self.link.callback(|_| Msg::Login);
        html! {
            <div class="container">
                <div class="form">
                    <div class="header">
                        <h2>{ "Login" }</h2>
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            label="E-Mail"
                            helper="email address"
                            helper_persistent=true
                            value=self.state.mail.clone()
                            oninput=mail_oninput />
                    </div>
                    <div class="form-item">
                        <MatTextField
                            classes=classes!("form-input")
                            outlined=true
                            field_type=TextFieldType::Password
                            label="Password"
                            helper="password"
                            helper_persistent=true
                            value=self.state.password.clone()
                            oninput=password_oninput />
                    </div>
                    {
                        if let Some(err) = &self.state.err {
                            html! {
                                <div class="error-info">
                                    <p>{ format!("Failed to login: {}", err) }</p>
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
                                label="Login"
                                disabled=self.need_to_disable()
                                raised=true />
                        </span>
                        <RouterAnchor<AppRoute>
                            route={ AppRoute::Register }
                            classes="form-row-item">
                            <MatButton
                                classes=classes!("form-button")
                                label="Register"
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
