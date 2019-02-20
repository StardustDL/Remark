use crate::document::Document;

pub mod html;

pub trait Converter {
    type TOutput;

    fn convert(document: &Document) -> Result<Self::TOutput, ()>;
}
