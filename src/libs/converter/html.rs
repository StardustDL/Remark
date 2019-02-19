use super::Converter;
use crate::{Document, DocumentItem::*};

pub struct HTMLConverter;

impl Converter for HTMLConverter {
    type TOutput = String;

    fn convert(document: &Document) -> Result<Self::TOutput, ()> {
        let mut body = String::new();

        for item in document {
            match item {
                Head(ri) => body.push_str(&format!("<h1> {} </h1>", ri.content)),
                Line(ri) => body.push_str(&format!("{} <br/>", ri.content)),
                _ => return Err(()),
            }
        }

        println!("{}", body);

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
