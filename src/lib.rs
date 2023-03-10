use crate::cube_parts::cube::{solved_cube, to_js, Cube};
use wasm_bindgen::prelude::*;

pub mod cube_parts;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_cube() -> JsValue {
    to_js()
}
