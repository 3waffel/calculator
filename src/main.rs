//! simple calculator

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

pub mod ast;

fn main() {
    raw_input();
}

fn raw_input() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read: {input}");
            let parse = grammar::ExprParser::new().parse(&input);
            if parse.is_ok() {
                println!("{:?}", parse.unwrap());
            } else {
                println!("{input} is an invalid expression.");
            }
        }
        Err(error) => {
            println!("error: {error}")
        }
    }
}

#[test]
fn test_bracket() {
    assert!(grammar::ExprParser::new().parse("22").is_ok());
    assert!(grammar::ExprParser::new().parse("(22)").is_ok());
    assert!(grammar::ExprParser::new().parse("((((22))))").is_ok());
    assert!(grammar::ExprParser::new().parse("((22)").is_err());
}

#[test]
fn test_expression() {
    let expr = grammar::ExprParser::new().parse("22 * 44 + 66").unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
