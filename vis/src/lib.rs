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
pub fn vis(input: String, output: String, t: i32) -> Ret {
    let input = tools::parse_input(&input);
    match tools::parse_output(&input, &output) {
        Ok(out) => {
            let (score, error, svg) = tools::vis(&input, &out[..t as usize], show_number, focus);
            Ret { score, error, svg }
        }
        Err(error) => Ret {
            score: 0,
            error,
            svg: "".to_owned(),
        },
    }
}

#[wasm_bindgen]
pub fn get_max_turn(input: String, output: String) -> i32 {
    let input = tools::parse_input(&input);
    if let Ok(out) = tools::parse_output(&input, &output) {
        out.len() as i32
    } else {
        0
    }
}
