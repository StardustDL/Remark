use super::Converter;
use crate::document::{Document, DocumentItem::*, HeadItem, LineItem, ParagraphItem};
use std::fmt::{self, Display, Formatter};

impl<'a> Display for LineItem<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<'a> Display for HeadItem<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "<h{level}> {content} </h{level}>",
            content = self.content,
            level = self.level
        )
    }
}

impl<'a> Display for ParagraphItem<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut body = String::new();

        for item in self {
            body.push_str(&format!("{} ", item));
        }

        write!(f, "<p> {content} </p>", content = body,)
    }
}

pub struct HTMLConverter;

impl Converter for HTMLConverter {
    type TOutput = String;

    fn convert(document: &Document) -> Result<Self::TOutput, ()> {
        let mut body = String::new();

        for item in document {
            match item {
                Head(ri) => body.push_str(&format!("{}", ri)),
                Paragraph(ri) => body.push_str(&format!("{}", ri)),
                // _ => unreachable!(),
            }
        }

        Ok(format!(
            "<!DOCTYPE html>
<html>
<body>
{}
</body>
</html>",
            body
        ))
    }
}
