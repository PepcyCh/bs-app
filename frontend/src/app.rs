use std::rc::Rc;

use yew::{agent::Bridged, Bridge, Component, ComponentLink, html};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::{
    components::{content::ContentComponent, login::LoginComponent, register::RegisterComponent},
    route::AppRoute,
};

pub struct App {
    link: ComponentLink<Self>,
    state: State,
    route_agent: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Default)]
struct State {
    mail: String,
    name: String,
    is_logged_in: bool,
}

pub enum Msg {
    Nop,
    Login((String, String)),
    Register,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| Msg::Nop));
        Self {
            link,
            state: State::default(),
            route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Nop => false,
            Msg::Login((mail, name)) => {
                self.state.is_logged_in = true;
                self.state.mail = mail;
                self.state.name = name;
                self.route_agent.send(ChangeRoute(AppRoute::Content.into()));
                true
            }
            Msg::Register => {
                self.route_agent.send(ChangeRoute(AppRoute::Login.into()));
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let login_callback = self
            .link
            .callback(|data: (String, String)| Msg::Login(data));
        let register_callback = self
            .link
            .callback(|_| Msg::Register);
        let mail = Rc::new(self.state.mail.clone());
        let name = Rc::new(self.state.name.clone());

        html! {
            <div>
                <Router<AppRoute, ()> render=Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Login | AppRoute::Home => html! {
                            <LoginComponent onlogin=login_callback.clone() />
                        },
                        AppRoute::Register => html! {
                            <RegisterComponent onregister=register_callback.clone() />
                        },
                        AppRoute::Content => html! {
                            <ContentComponent mail=mail.clone() name=name.clone() />
                        }
                    }
                })
                />
            </div>
        }
    }
}
