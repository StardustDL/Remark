#[derive(Parser)]
#[grammar = "libs/pests/main.pest"]
pub struct InnerParser;

#[cfg(test)]
mod tests {
    use super::{InnerParser, Rule};
    use pest::Parser;

    #[test]
    fn head() {
        assert!(InnerParser::parse(Rule::file, "# abc中文\n").is_err());
        assert!(InnerParser::parse(Rule::file, "## abc中文\n\n").is_err());
        assert!(InnerParser::parse(Rule::file, "# \n\n").is_err()); // No empty title

        let file = InnerParser::parse(Rule::file, "# abc中文\n\n")
            .expect("parse failed")
            .next()
            .unwrap();
        let item = file.into_inner().next().unwrap();
        assert_eq!(item.as_rule(), Rule::head);
        assert_eq!(item.into_inner().next().unwrap().as_str(), "abc中文");
    }
}
