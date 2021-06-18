use crate::fluent;
use fluent_templates::{static_loader, LanguageIdentifier, Loader};
use std::borrow::Cow;
use yew::{classes, html, Callback, Children, Classes, Component, ComponentLink, Properties};
use yew_material::MatButton;

static_loader! {
    static LOCALES = {
        locales: "./text/paged_list",
        fallback_language: "zh-CN",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub struct PagedList {
    link: ComponentLink<Self>,
    state: State,
    props: PagedListProps,
}

struct State {
    curr_page: usize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PagedListProps {
    pub lang_id: LanguageIdentifier,
    #[prop_or_default]
    pub classes: Classes,
    pub page_size: usize,
    pub items_count: usize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_page_changed: Callback<(usize, usize)>,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    FirstPage,
    LastPage,
    NextPage,
    PreviousPage,
}

impl Component for PagedList {
    type Message = Msg;
    type Properties = PagedListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State { curr_page: 0 },
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let page_count = (self.props.items_count + self.props.page_size - 1) / self.props.page_size;
        match msg {
            Msg::FirstPage => {
                self.state.curr_page = 0;
            }
            Msg::LastPage => {
                self.state.curr_page = page_count - 1;
            }
            Msg::NextPage => {
                self.state.curr_page = (self.state.curr_page + 1).min(page_count - 1);
            }
            Msg::PreviousPage => {
                self.state.curr_page = self.state.curr_page.max(1) - 1;
            }
        }
        self.props
            .on_page_changed
            .emit((self.state.curr_page, self.props.page_size));
        true
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
        let class_str = format!("paged-list {}", self.props.classes.to_string());

        html! {
            <div class=class_str>
                { for self.props.children.iter() }
                { self.page_buttons() }
            </div>
        }
    }
}

impl PagedList {
    fn page_buttons(&self) -> yew::Html {
        let first_page_click = self.link.callback(|_| Msg::FirstPage);
        let last_page_click = self.link.callback(|_| Msg::LastPage);
        let next_page_click = self.link.callback(|_| Msg::NextPage);
        let previous_page_click = self.link.callback(|_| Msg::PreviousPage);

        let page_count =
            ((self.props.items_count + self.props.page_size - 1) / self.props.page_size).max(1);
        let prev_disabled = self.state.curr_page == 0 || self.props.disabled;
        let next_disabled = self.state.curr_page == page_count - 1 || self.props.disabled;

        html! {
            <div class="page-buttons">
                <span
                    class="page-buttons-item"
                    onclick=first_page_click
                    disabled=prev_disabled >
                    <MatButton
                        classes=classes!("page-button")
                        label=fluent!(self.props.lang_id, "first-page")
                        icon=Cow::from("first_page")
                        outlined=true />
                </span>
                <span
                    class="page-buttons-item"
                    onclick=previous_page_click
                    disabled=prev_disabled >
                    <MatButton
                    classes=classes!("page-button")
                        label=fluent!(self.props.lang_id, "prev-page")
                        icon=Cow::from("arrow_back_ios")
                        outlined=true />
                </span>
                <span class="page-buttons-item">
                    { fluent!(self.props.lang_id, "page-hint", {
                        "curr" => self.state.curr_page + 1,
                        "total" => page_count,
                    }) }
                </span>
                <span
                    class="page-buttons-item"
                    onclick=next_page_click
                    disabled=next_disabled >
                    <MatButton
                    classes=classes!("page-button")
                        label=fluent!(self.props.lang_id, "next-page")
                        icon=Cow::from("arrow_forward_ios")
                        trailing_icon=true
                        outlined=true />
                </span>
                <span
                    class="page-buttons-item"
                    onclick=last_page_click
                    disabled=next_disabled >
                    <MatButton
                    classes=classes!("page-button")
                        label=fluent!(self.props.lang_id, "last-page")
                        icon=Cow::from("last_page")
                        trailing_icon=true
                        outlined=true />
                </span>
            </div>
        }
    }
}
