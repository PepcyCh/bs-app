use crate::route::AppRoute;
use common::{
    request::RegisterRequest,
    response::{ErrorResponse, SimpleResponse},
};
use lazy_static::lazy_static;
use regex::Regex;
use yew::{
    agent::Bridged,
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Bridge, Callback, Component, ComponentLink, InputData, Properties,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

pub struct RegisterComponent {
    link: ComponentLink<Self>,
    props: Prop,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
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
pub struct Prop {
    pub onregister: Callback<()>,
}

lazy_static! {
    static ref MAIL_RE: Regex =
        Regex::new(r"^[0-9a-zA-Z._+-]+@[0-9a-zA-Z-]+\.[0-9a-zA-Z-.]+$").unwrap();
    static ref NAME_RE: Regex = Regex::new(r"^[0-9a-zA-Z_]{3, 32}$").unwrap();
    static ref PASSWORD_RE: Regex = Regex::new(r"^[0-9a-zA-Z_]{6, 32}$").unwrap();
}

impl Component for RegisterComponent {
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
                    self.state.err = Some("Invalid mail address".to_string());
                } else if !NAME_RE.is_match(&self.state.name) {
                    self.state.err = Some("Invalid username".to_string());
                } else if !PASSWORD_RE.is_match(&self.state.password) {
                    self.state.err = Some("Invalid password".to_string());
                } else if self.state.password != self.state.password_twice {
                    self.state.err = Some("The 2 passwords are different".to_string());
                } else {
                    let register_info = RegisterRequest {
                        mail: self.state.mail.clone(),
                        name: self.state.name.clone(),
                        password: self.state.password.clone(),
                    };
                    let body = serde_json::to_value(&register_info).unwrap();
                    let request = Request::post("/register")
                        .header("Content-Type", "application/json")
                        .body(Json(&body))
                        .expect("Failed to construct register request");
                    let callback = self.link.callback(
                        |response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                            let Json(data) = response.into_body();
                            if let Ok(response) = data {
                                Msg::RegisterResponse(response)
                            } else {
                                Msg::RegisterResponse(SimpleResponse::err("Unknown error"))
                            }
                        },
                    );
                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
                }
                true
            }
            Msg::RegisterResponse(response) => {
                self.fetch_task = None;
                if response.success {
                    self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                    self.props.onregister.emit(());
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
        let name_oninput = self.link.callback(|e: InputData| Msg::EditName(e.value));
        let password_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPassword(e.value));
        let password2_oninput = self
            .link
            .callback(|e: InputData| Msg::EditPasswordTwice(e.value));
        let register_click = self.link.callback(|_| Msg::Register);
        html! {
            <div>
                <h1>{ "Register" }</h1>
                <div>
                    { "Mail: " }
                    <input
                        placeholder="Mail"
                        value=&self.state.mail
                        oninput=mail_oninput />
                    { "Username: " }
                    <input
                        placeholder="Username"
                        value=&self.state.name
                        oninput=name_oninput />
                    { "Password: " }
                    <input
                        placeholder="Password"
                        type="password"
                        value=&self.state.password
                        oninput=password_oninput />
                    { "Password (twice): " }
                    <input
                        placeholder="Password"
                        type="password"
                        value=&self.state.password_twice
                        oninput=password2_oninput />
                    <button
                        onclick=register_click
                        disabled=self.fetch_task.is_some() >
                        { "Register" }
                    </button>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div>
                                <p>{ format!("Failed to register: {}", err) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <RouterAnchor<AppRoute> route={ AppRoute::Login }>
                    { "Login" }
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
