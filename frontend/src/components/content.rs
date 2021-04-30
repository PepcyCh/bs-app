use std::rc::Rc;

use yew::{html, Component, ComponentLink, Properties};

pub struct ContentComponent {
    _link: ComponentLink<Self>,
    props: Prop,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Prop {
    pub mail: Rc<String>,
    pub name: Rc<String>,
}

impl Component for ContentComponent {
    type Message = ();
    type Properties = Prop;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <h1>{ "Content" }</h1>
                <p>{ format!("Welcome, {}({})!", self.props.name, self.props.mail) }</p>
                <p>{ "TODO" }</p>
            </div>
        }
    }
}
