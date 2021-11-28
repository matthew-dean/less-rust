use wasm_bindgen::prelude::*;

use pest::Parser;

#[derive(Parser)]
#[grammar = "less.pest"]
pub struct LessParser;

#[wasm_bindgen]
pub fn parse() {
    let successful_parse = LessParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = LessParser::parse(Rule::field, "this is not a number");
    println!("{:?}", unsuccessful_parse);
}