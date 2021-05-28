use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::{Component, Properties, virtual_dom::VNode};
use web_sys::Node;

pub struct LineChart {
    node: Node,
    props: LineChartProps,
}

pub struct LineChartData {
    pub x: f64,
    pub y: f64,
}

#[derive(Properties, Clone)]
pub struct LineChartProps {
    pub height: u32,
    pub data: Rc<Vec<LineChartData>>,
}

impl Component for LineChart {
    type Message = ();
    type Properties = LineChartProps;

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
        let data_js = js_sys::Array::new();
        for data in self.props.data.iter() {
            let item_js = js_sys::Object::new();
            js_sys::Reflect::set(&item_js, &JsValue::from_str("x"), &JsValue::from_f64(data.x)).unwrap();
            js_sys::Reflect::set(&item_js, &JsValue::from_str("y"), &JsValue::from_f64(data.y)).unwrap();
            data_js.push(&item_js);
        }
        
        render_line_chart(&self.node, self.props.height, &data_js);

        VNode::VRef(self.node.clone())
    }
}

#[wasm_bindgen(module = "/src/utils/line_chart.js")]
extern "C" {
    fn render_line_chart(node: &Node, height: u32, data: &JsValue);
}
