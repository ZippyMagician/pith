mod lexer;
mod parser;
mod tokens;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(program: &str, input: &[u8]) {
    parser::parse(
        &parser::pre_process(lexer::tokenize(program)),
        input.to_owned(),
    );
}
