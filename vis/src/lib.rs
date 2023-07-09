#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ret {
    pub score: i64,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
    #[wasm_bindgen(getter_with_clone)]
    pub svg: String,
}

#[wasm_bindgen]
pub fn vis(
    input: String,
    output: String,
    _t: i32,
    color_type: i32,
    focus: i32,
    color_instrument: i32,
) -> Ret {
    let input = icfpc2023::parse_input(&input);
    let out = icfpc2023::parse_output_or_die(&output);
    let (score, error, svg) = icfpc2023::vis::vis(
        &input,
        &out,
        color_type,
        if focus < 0 { !0 } else { focus as usize },
        (color_instrument >= 0).then_some(color_instrument as usize),
    );
    Ret { score, error, svg }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> i32 {
    1
}
