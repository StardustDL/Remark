use pest::Parser;

mod inner;
use inner::{InnerParser, Rule};

use crate::{Document, DocumentItem::*, HeadItem, LineItem};

pub fn parse<'i>(input: &'i str) -> Result<Document, ()> {
    let file = match InnerParser::parse(Rule::file, input) {
        Ok(mut res) => res.next().unwrap(),
        Err(_) => {
            return Err(());
        }
    };
    let mut items = Vec::new();

    for item in file.into_inner() {
        match item.as_rule() {
            Rule::head => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(), // line.as_str
            })),
            Rule::line => items.push(Line(LineItem {
                content: item.as_str(),
            })),
            Rule::EOI => (),
            _ => panic!(format!("{:#?}", item)),
        }
    }

    Ok(Document { items })
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::DocumentItem::*;

    #[test]
    fn head() {
        assert!(parse("# abc中文\n").is_err());
        assert!(parse("## abc中文\n\n").is_err());
        assert!(parse("# \n\n").is_err()); // No empty title

        let doc = parse("# abc中文\n\n").expect("parse failed");

        for item in doc {
            match item {
                Head(vi) => assert_eq!(vi.content, "abc中文"),
                _ => panic!("!"),
            }
        }
    }
}
