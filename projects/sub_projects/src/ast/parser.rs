use crate::{Result, AST, ParserConfig};
use wolfram_pest::{Rule, Pairs, WolframParser, Parser, Pair};
use std::collections::HashMap;

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParserConfig {
    pub fn parse(&self, text: &str) -> Result<AST> {
        let input = text.replace("\r\n", "\n");
        let pairs = WolframParser::parse(Rule::program, &input)?;
        Ok(self.parse_program(pairs))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::Integer => AST::Integer(0),
                Rule::expression=> self.parse_expression(pair),
                _ => debug_cases!(pair),
            };
            // println!("{:?}", code);
            codes.push(code);
        }
        AST::Graphics(codes, HashMap::new())
    }
    fn parse_expression(&self, pairs: Pair<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::Integer => AST::Integer(0),

                _ => debug_cases!(pair),
            };
            // println!("{:?}", code);
            codes.push(code);
        }
        AST::Graphics(codes, HashMap::new())
    }


}