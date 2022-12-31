use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, AttrValue, Classes, Html,
    Properties,
};

use crate::app::log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = MathJax, js_name = "typeset")]
    fn math_jax_typeset(nodes: Box<[JsValue]>);

    #[wasm_bindgen(js_namespace = MathJax, js_name = "typesetClear")]
    fn math_jax_typeset_clear(nodes: Box<[JsValue]>);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expression: AttrValue,
    #[prop_or_default]
    pub centered: bool,
}

#[function_component]
pub fn Math(
    Props {
        expression,
        centered,
    }: &Props,
) -> Html {
    let node_ref = use_node_ref();
    {
        let node_ref = node_ref.clone();
        let expression = expression.clone();
        use_effect_with_deps(
            move |_| {
                log("Performing use_effect_with_deps");
                math_jax_typeset(Box::new([node_ref
                    .get()
                    .unwrap()
                    .into_js_result()
                    .unwrap()]))
            },
            expression,
        );
    }
    let mut class = Classes::new();
    if *centered {
        class.push("centered");
    }
    match node_ref.get() {
        Some(node) => {
            log("Non-empty node_ref");
            math_jax_typeset_clear(Box::new([(&node).into_js_result().unwrap()]));
            node.set_text_content(Some(&format!(r"\({expression}\)")));

            html! {<div ref={node_ref} {class}>
                {r"\("}{expression}{r"\)"}
            </div>}
        }
        None => {
            log("Empty node_ref");

            html! {<div ref={node_ref} {class}>
                {r"\("}{expression}{r"\)"}
            </div>}
        }
    }
}
