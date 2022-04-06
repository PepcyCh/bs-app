use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::{virtual_dom::VNode, Component, Properties};

pub struct Map {
    node: Node,
    props: MapProps,
}

pub struct MapPointData {
    pub x: f64,
    pub y: f64,
    pub value: f64,
}

#[derive(Properties, Clone)]
pub struct MapProps {
    pub data: Rc<Vec<MapPointData>>,
}

impl Component for Map {
    type Message = ();
    type Properties = MapProps;

    fn create(props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self {
            node: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap()
                .into(),
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        //rust改成js格式。
        let data_js = js_sys::Array::new();
        for data in self.props.data.iter() {
            let item_js = js_sys::Object::new();
            js_sys::Reflect::set(
                &item_js,
                &JsValue::from_str("x"),
                &JsValue::from_f64(data.x), // 自己的property 
            )
            .unwrap();
            js_sys::Reflect::set(
                &item_js,
                &JsValue::from_str("y"),
                &JsValue::from_f64(data.y),
            )
            .unwrap();
            js_sys::Reflect::set(
                &item_js,
                &JsValue::from_str("value"),
                &JsValue::from_f64(data.value),
            )
            .unwrap();
            data_js.push(&item_js);
        }

        render_map(&self.node, &data_js);// 调用js的render map函数。

        VNode::VRef(self.node.clone())
    }
}

#[wasm_bindgen(module = "/js/dist/bundle.js")]
extern "C" {
    fn render_map(node: &Node, data: &JsValue);//和js绑定。 
}
