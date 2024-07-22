use pest::{error::Error, iterators::Pairs, Parser};
use pest_derive::Parser;

mod ast;
mod ast_2;

#[derive(Parser)]
#[grammar = "../request.pest"]
pub struct RequestParser;

impl RequestParser {
    pub fn parse_request(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        RequestParser::parse(Rule::request, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_request_example_with_recursion() {
        let input = include_str!("../get_example.http");
        let pairs = RequestParser::parse_request(input).unwrap_or_else(|e| panic!("{}", e));
        let mut indentation = String::new();

        fn dive_in(pairs: Pairs<Rule>, indentation: &mut String) {
            for pair in pairs {
                let next_pair = pair.clone().into_inner();
                if next_pair.clone().count() > 0 {
                    println!("{indentation}- {:?}", pair.as_rule());
                    indentation.push(' ');
                    indentation.push(' ');
                    dive_in(next_pair, indentation);
                } else {
                    println!("{indentation}- {:?}: {:?}", pair.as_rule(), pair.as_str());
                }
            }
        }

        dive_in(pairs, &mut indentation);
    }

    #[test]
    fn test_get_request_example_with_rule_matching() {
        let input = include_str!("../get_example.http");
        let pairs = RequestParser::parse_request(input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            match pair.as_rule() {
                Rule::method => {
                    println!("- {:?}: {:?} ", pair.as_rule(), pair.as_str());
                    assert_eq!(pair.as_str(), "GET");
                }
                Rule::url => {
                    println!("- {:?} ({})", pair.as_rule(), pair.as_str());
                    for url_component in pair.into_inner() {
                        println!(
                            "  - {:?}: {:?} ",
                            url_component.as_rule(),
                            url_component.as_str()
                        );
                        match url_component.as_rule() {
                            Rule::protocol => {
                                assert_eq!(url_component.as_str(), "http");
                            }
                            Rule::host => {
                                assert_eq!(url_component.as_str(), "foo.de");
                            }
                            Rule::port => {
                                assert_eq!(url_component.as_str(), "9000");
                            }
                            Rule::path => {
                                assert_eq!(url_component.as_str(), "/path/b/c");
                            }
                            _ => {}
                        }
                    }
                }
                Rule::header => {
                    println!("- {:?}: {:?} ", pair.as_rule(), pair.as_str());
                    let mut inner_pairs = pair.into_inner();
                    let key = inner_pairs.next().unwrap();
                    let value = inner_pairs.next().unwrap();
                    assert_eq!(key.as_str(), "Accept");
                    assert_eq!(value.as_str(), "text/html");
                }
                _ => {}
            }
        }
    }
}
