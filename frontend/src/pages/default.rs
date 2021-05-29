use std::rc::Rc;

use common::response::{ErrorResponse, SimpleResponse};
use yew::{
    agent::Bridged,
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Bridge, Component, Properties,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};

use crate::route::AppRoute;

pub struct DefaultComponent {
    route_agent: Box<dyn Bridge<RouteAgent>>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Nop,
    CheckResponse(SimpleResponse),
    ToLogin,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub login_token: Rc<String>,
}

impl Component for DefaultComponent {
    type Message = Msg;
    type Properties = Prop;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let fetch_task = if !props.login_token.is_empty() {
            let login_token = (*props.login_token).clone();
            let body = serde_json::to_value(login_token).unwrap();
            let request = Request::post("/check_login")
                .header("Content-Type", "application/json")
                .body(Json(&body))
                .expect("Failed to construct check login request");
            let callback =
                link.callback(|response: Response<Json<anyhow::Result<SimpleResponse>>>| {
                    let Json(data) = response.into_body();
                    if let Ok(result) = data {
                        Msg::CheckResponse(result)
                    } else {
                        Msg::CheckResponse(SimpleResponse::err("Unknown error"))
                    }
                });
            let task = FetchService::fetch(request, callback).expect("Failed to start request");
            Some(task)
        } else {
            None
        };

        let mut component = Self {
            route_agent,
            fetch_task,
        };
        if component.fetch_task.is_none() {
            component.update(Msg::ToLogin);
        }
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::CheckResponse(response) => {
                if response.success {
                    self.route_agent.send(ChangeRoute(AppRoute::Home.into()));
                } else {
                    self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                }
                true
            }
            Msg::ToLogin => {
                self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {}
    }
}
