use crate::{
    protocol::{LoginInfo, LoginResponse},
    route::AppRoute,
};
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
                if !self.state.mail.is_empty() && !self.state.password.is_empty() {
                    self.state.err = None;
                    let login_info = LoginInfo {
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
                                Msg::LoginResponse(LoginResponse {
                                    success: false,
                                    err: "Unknown error".to_string(),
                                    mail: "".to_string(),
                                    name: "".to_string(),
                                })
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
                    self.route_agent.send(ChangeRoute(AppRoute::Content.into()));
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
            <div>
                <h1>{ "Login" }</h1>
                <div>
                    { "Mail: " }
                    <input
                        placeholder="Mail"
                        value=&self.state.mail
                        oninput=mail_oninput />
                    { "Password: " }
                    <input
                        placeholder="Password"
                        type="password"
                        value=&self.state.password
                        oninput=password_oninput />
                    <button
                        onclick=login_click
                        disabled=self.fetch_task.is_some() >
                        { "Login" }
                    </button>
                </div>
                {
                    if let Some(err) = &self.state.err {
                        html! {
                            <div>
                                <p>{ format!("Failed to login: {}", err) }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <RouterAnchor<AppRoute> route={ AppRoute::Register }>
                    { "Register" }
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
