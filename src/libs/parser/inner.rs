#[derive(Parser)]
#[grammar = "libs/pests/main.pest"]
pub struct InnerParser;

#[cfg(test)]
mod tests {
    use super::{InnerParser, Rule};
    use pest::Parser;

    #[test]
    fn head() {
        assert!(InnerParser::parse(Rule::head1, "## abc中文\n\n").is_err());

        let head = InnerParser::parse(Rule::head, "# abc中文\n\n")
            .expect("parse failed")
            .next()
            .unwrap();
        assert_eq!(head.into_inner().next().unwrap().as_str(), "abc中文");
    }
}
