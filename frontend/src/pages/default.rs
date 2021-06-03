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
    Bridge, Component, ComponentLink, Properties,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};

use crate::route::AppRoute;

pub struct DefaultComponent {
    link: ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    props: Props,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    Nop,
    Check,
    CheckResponse(SimpleResponse),
    ToLogin,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub login_token: Rc<String>,
}

impl Component for DefaultComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let mut component = Self {
            link,
            route_agent,
            props,
            fetch_task: None,
        };
        if !component.props.login_token.is_empty() {
            component.update(Msg::Check);
        } else {
            component.update(Msg::ToLogin);
        }
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::Check => {
                let login_token = (*self.props.login_token).clone();
                crate::create_fetch_task!(self, "/check_login", login_token, CheckResponse);
                true
            }
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
