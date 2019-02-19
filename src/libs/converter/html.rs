use super::Converter;
use crate::{Document, DocumentItem::*};

pub struct HTMLConverter;

impl Converter for HTMLConverter {
    type TOutput = String;

    fn convert(document: &Document) -> Result<Self::TOutput, ()> {
        let mut body = String::new();

        for item in document {
            match item {
                Head(ri) => body.push_str(&format!("<h{level}> {content} </h{level}>", content = ri.content, level = ri.level)),
                Line(ri) => body.push_str(&format!("{} <br/>", ri.content)),
                // _ => return Err(()),
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
