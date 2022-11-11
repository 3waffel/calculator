use dioxus::events::KeyCode;
use dioxus::prelude::*;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);
pub mod ast;

fn main() {
    dioxus_tui::launch(app);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Panel {
    Record,
    Input,
}

impl Panel {
    pub fn flip(self) -> Self {
        match self {
            Panel::Record => Panel::Input,
            Panel::Input => Panel::Record,
        }
    }
}

fn app(cx: Scope) -> Element {
    let mut records = vec!["1", "2", "3"];
    let focusing = use_state(&cx, || Panel::Input);

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            height: "100%",
            onkeydown: move |e| {
                match e.data.key_code {
                    KeyCode::LeftArrow => {
                        if focusing.get() == &Panel::Input {
                            focusing.set(Panel::Record);
                        }
                    }
                    KeyCode::RightArrow => {
                        if focusing.get() == &Panel::Record {
                            focusing.set(Panel::Input);
                        }
                    }
                    _ => {

                    }
                }
            },
            div {
                div {
                    border_style: "solid",
                    border_color: format_args!("{}",
                        if focusing.get() == &Panel::Record {"cyan"} else {"default"}
                    ),
                    justify_content: "center",
                    align_items: "center",
                    "Record Area",
                    ul {
                        {records.iter().map(|f| rsx!(li {"a"}))}
                    }
                },
                div {
                    border_style: "solid",
                    border_color: format_args!("{}",
                        if focusing.get() == &Panel::Input {"cyan"} else {"default"}
                    ),
                    justify_content: "center",
                    align_items: "center",
                    "Input Area",
                    button {
                        border_style: "solid",
                        onclick: move |e| {},
                        justify_content: "center",
                        align_items: "center",
                        display: "flex",
                        flex_direction: "column",
                        "Enter"
                    }
                }
            }
        }
    })
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
