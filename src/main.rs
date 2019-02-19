use remark::{
    converter::{html::HTMLConverter, Converter},
    parser,
};

fn main() {
    let input = "# H1

## H2

### H3


This is a sentence.
";
    let doc = parser::parse(input).unwrap();
    println!("{}", HTMLConverter::convert(&doc).unwrap());
}
