use wasm_bindgen::prelude::*;

use pest::Parser;

#[derive(Parser)]
#[grammar = "less.pest"]
pub struct LessParser;

enum LessValue<'a> {
    Object(Vec<(&'a str, LessValue<'a>)>),
    Array(Vec<LessValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

use pest::error::Error;

#[wasm_bindgen]
pub fn parse(contents: &str) -> Result<LessValue, Error<Rule>>{
    let less = LessParser::parse(Rule::root, contents)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> LessValue {
        match pair.as_rule() {
            Rule::object => LessValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => LessValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => LessValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => LessValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => LessValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => LessValue::Null,
            Rule::root
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(less))
}