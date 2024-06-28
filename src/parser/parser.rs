use pest::{iterators::*, Parser};
use pest_derive::*;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct ExpressionParser;
