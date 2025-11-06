use crate::evaluator::eval;
use crate::parser::parse_form;

mod evaluator;
mod parser;
mod types;

pub fn process_str(line: &str) -> Result<f32, String> {
    match parse_form(line) {
        Ok((_input, parsed_line)) => Ok(eval(parsed_line)),
        Err(error) => Err(error.to_string()),
    }
}
