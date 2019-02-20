use super::inner::Rule;
use crate::document::{
    EmphasizedItem, HeadItem, InlineItem::*, LineItem, ParagraphItem, StrongItem, TextItem,
};
use pest::iterators::Pair;

#[inline]
fn inner_first(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
}

pub fn parse_head(source: Pair<Rule>) -> HeadItem {
    match source.as_rule() {
        Rule::head1 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 1,
        },
        Rule::head2 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 2,
        },
        Rule::head3 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 3,
        },
        Rule::head4 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 4,
        },
        Rule::head5 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 5,
        },
        Rule::head6 => HeadItem {
            content: parse_line(inner_first(source)),
            level: 6,
        },
        _ => unreachable!(),
    }
}

pub fn parse_line(source: Pair<Rule>) -> LineItem {
    match source.as_rule() {
        Rule::line => parse_line(inner_first(source)),
        Rule::line_ne => {
            let mut items = Vec::new();
            for item in source.into_inner() {
                match item.as_rule() {
                    Rule::strong => items.push(Strong(StrongItem(inner_first(item).as_str()))),
                    Rule::emphasized => {
                        items.push(Emphasized(EmphasizedItem(inner_first(item).as_str())))
                    }
                    Rule::word | Rule::white_space => items.push(Text(TextItem(item.as_str()))),
                    _ => unreachable!(),
                }
            }
            LineItem { items }
        }
        Rule::line_e => LineItem { items: vec![] },
        _ => unreachable!(),
    }
}

pub fn parse_paragraph(source: Pair<Rule>) -> ParagraphItem {
    match source.as_rule() {
        Rule::paragraph => {
            let mut items = Vec::new();
            for item in source.into_inner() {
                match item.as_rule() {
                    Rule::line_ne => items.push(parse_line(item)),
                    _ => unreachable!(),
                }
            }
            ParagraphItem { items }
        }
        _ => unreachable!(),
    }
}
