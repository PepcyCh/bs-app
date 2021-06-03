use yew::{html, Callback, Children, Classes, Component, Properties};

pub struct CardDiv {
    props: CardDivProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardDivProps {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for CardDiv {
    type Message = ();
    type Properties = CardDivProps;

    fn create(props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self { props }
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
        let class_str = format!("card-div {}", self.props.classes.to_string());

        html! {
            <div class=class_str>
                { for self.props.children.iter() }
            </div>
        }
    }
}
