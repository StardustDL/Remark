#[derive(Debug)]
pub enum DocumentItem<'a> {
    Head(HeadItem<'a>),
    Paragraph(ParagraphItem<'a>),
}

#[derive(Debug)]
pub struct HeadItem<'a> {
    pub content: LineItem<'a>,
    pub level: u8,
}

#[derive(Debug)]
pub struct ParagraphItem<'a> {
    pub items: Vec<LineItem<'a>>,
}

impl<'a> IntoIterator for ParagraphItem<'a> {
    type Item = LineItem<'a>;
    type IntoIter = <Vec<LineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, 'i> IntoIterator for &'i ParagraphItem<'a> {
    type Item = &'i LineItem<'a>;
    type IntoIter = <&'i Vec<LineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, 'i> IntoIterator for &'i mut ParagraphItem<'a> {
    type Item = &'i mut LineItem<'a>;
    type IntoIter = <&'i mut Vec<LineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

#[derive(Debug)]
pub enum InlineItem<'a> {
    Text(TextItem<'a>),
    Emphasized(EmphasizedItem<'a>),
    Strong(StrongItem<'a>),
}

#[derive(Debug)]
pub struct StrongItem<'a>(pub &'a str);

#[derive(Debug)]
pub struct EmphasizedItem<'a>(pub &'a str);

#[derive(Debug)]
pub struct TextItem<'a>(pub &'a str);

#[derive(Debug)]
pub struct LineItem<'a> {
    pub items: Vec<InlineItem<'a>>,
}

impl<'a> IntoIterator for LineItem<'a> {
    type Item = InlineItem<'a>;
    type IntoIter = <Vec<InlineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, 'i> IntoIterator for &'i LineItem<'a> {
    type Item = &'i InlineItem<'a>;
    type IntoIter = <&'i Vec<InlineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, 'i> IntoIterator for &'i mut LineItem<'a> {
    type Item = &'i mut InlineItem<'a>;
    type IntoIter = <&'i mut Vec<InlineItem<'a>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

#[derive(Debug)]
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
