use yew::{agent::Bridged, html, Bridge, Component};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::RouteAgent};

use crate::route::AppRoute;

pub struct DefaultComponent {
    route_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Nop,
    ToLogin,
    ToHome,
}

impl Component for DefaultComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        let mut component = Self { route_agent };
        // TODO - check cookie or some other things (maybe a fetch task is needed)
        component.update(Msg::ToLogin);
        component
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::ToLogin => {
                self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
            Msg::ToHome => {
                self.route_agent.send(ChangeRoute(AppRoute::Home.into()));
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
