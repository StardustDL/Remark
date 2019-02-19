use remark::{
    converter::{html::HTMLConverter, Converter},
    parser,
};

fn main() {
    let input = "# abc

This is a sentence.
";
    let doc = parser::parse(input).unwrap();

    println!("{}", HTMLConverter::convert(&doc).unwrap());
}
