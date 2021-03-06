use crate::fluent;
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use std::time::Duration;
use yew::{
    html,
    services::{timeout::TimeoutTask, TimeoutService},
    Callback, Component, ComponentLink, Properties,
};
use yew_material::MatButton;

static_loader! {
    static LOCALES = {
        locales: "./text/logout_hint",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct LogoutHint {
    link: ComponentLink<Self>,
    props: Props,
    timeout_task: Option<TimeoutTask>,
}

pub enum Msg {
    Logout,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub lang_id: LanguageIdentifier,
    pub onlogout: Callback<()>,
}

impl Component for LogoutHint {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let timeout_task =
            TimeoutService::spawn(Duration::new(3, 0), link.callback(|_| Msg::Logout));
        Self {
            link,
            props,
            timeout_task: Some(timeout_task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Logout => {
                if let Some(timeout_task) = std::mem::replace(&mut self.timeout_task, None) {
                    drop(timeout_task);
                }
                self.props.onlogout.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        let logout_callback = self.link.callback(|_| Msg::Logout);
        html! {
            <div class="container">
                <div class="logout-hint">
                    <p class="error-info">{ fluent!(self.props.lang_id, "info") }</p>
                    <span onclick=logout_callback>
                        <MatButton label=fluent!(self.props.lang_id, "button-label") raised=true />
                    </span>
                </div>
            </div>
        }
    }
}
