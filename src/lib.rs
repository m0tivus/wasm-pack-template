mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main(x1: &JsValue, x2: &JsValue) -> String {
    let _x1: i32 = x1.into_serde().unwrap();
    let _x2: i32 = x2.into_serde().unwrap();
    return format!("Hello {{project-name}}! {:?}", _x1 + _x2);
}
