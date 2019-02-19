use pest::Parser;

mod inner;
use inner::{InnerParser, Rule};

use crate::{Document, DocumentItem::*, HeadItem, LineItem};

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
            Rule::head1 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 1,
            })),
            Rule::head2 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 2,
            })),
            Rule::head3 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 3,
            })),
            Rule::head4 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 4,
            })),
            Rule::head5 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 5,
            })),
            Rule::head6 => items.push(Head(HeadItem {
                content: item.into_inner().next().unwrap().as_str(),
                level: 6,
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
        let doc = parse("# abc中文\n").expect("parse failed");

        for item in doc {
            match item {
                Head(vi) => assert_eq!(vi.content, "abc中文"),
                _ => panic!("No head"),
            }
        }
    }
}
