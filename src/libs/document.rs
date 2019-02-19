pub enum DocumentItem<'a> {
    Head(HeadItem<'a>),
    Line(LineItem<'a>),
}

pub struct HeadItem<'a> {
    pub content: &'a str,
    pub level: u8,
}

pub struct LineItem<'a> {
    pub content: &'a str,
}

pub struct Document<'a> {
    pub items: Vec<DocumentItem<'a>>,
}

impl<'a> IntoIterator for Document<'a> {
    type Item = DocumentItem<'a>;
    type IntoIter = <Vec<DocumentItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, 'i> IntoIterator for &'i Document<'a> {
    type Item = &'i DocumentItem<'a>;
    type IntoIter = <&'i Vec<DocumentItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, 'i> IntoIterator for &'i mut Document<'a> {
    type Item = &'i mut DocumentItem<'a>;
    type IntoIter = <&'i mut Vec<DocumentItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}
