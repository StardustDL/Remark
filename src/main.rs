use remark::{
    converter::{html::HTMLConverter, Converter},
    parser,
};

fn main() {
    let input = "# H1

## H2

### H3


This is a paragraph.
This is the first sentence.
This is the second sentence.

This is another paragraph.
This is the first sentence.
This is the second sentence.";
    let doc = parser::parse(input).unwrap();

    dbg!(&doc);

    println!("{}", HTMLConverter::convert(&doc).unwrap());
}
