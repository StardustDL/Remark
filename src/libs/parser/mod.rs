use pest::Parser;

pub mod inner;
use inner::{InnerParser, Rule};

mod item_parser;
use item_parser::*;

use crate::document::{Document, DocumentItem::*};

pub fn parse(input: &str) -> Result<Document, ()> {
    let file = match InnerParser::parse(Rule::file, input) {
        Ok(mut res) => res.next().unwrap(),
        Err(er) => {
            dbg!(er);
            return Err(());
        }
    };
    let mut items = Vec::new();

    for item in file.into_inner() {
        match item.as_rule() {
            Rule::head1 | Rule::head2 | Rule::head3 | Rule::head4 | Rule::head5 | Rule::head6 => {
                items.push(Head(parse_head(item)))
            }
            Rule::paragraph => items.push(Paragraph(parse_paragraph(item))),
            Rule::EOI => (),
            _ => unreachable!(format!("{:#?}", item)),
        }
    }

    Ok(Document { items })
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::document::{DocumentItem::*, InlineItem::*};

    #[test]
    fn head() {
        let doc = parse("# abc中文\n").expect("parse failed");

        for item in doc {
            match item {
                Head(vi) => {
                    let inn = vi.content.into_iter().next().unwrap();
                    match inn {
                        Text(ti) => assert_eq!(ti.0, "abc中文"),
                        _ => panic!("No head"),
                    }
                }
                _ => panic!("No head"),
            }
        }
    }
}
