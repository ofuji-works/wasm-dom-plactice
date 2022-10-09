use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Node;

#[wasm_bindgen]
pub fn get_element_children(elem: Node) -> Node {
    let result = elem.first_child();
    let child = match result {
        Option::Some(e) => e,
        Option::None => panic!("unexpected result"),
    };

    child
}
