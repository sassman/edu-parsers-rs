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
        let pairs = RequestParser::parse_request(input).unwrap();

        fn dive_in(pairs: Pairs<Rule>, indentation: &str) {
            for pair in pairs {
                let next_pair = pair.clone().into_inner();
                if next_pair.clone().count() > 0 {
                    println!("{indentation}- {:?}", pair.as_rule());
                    let mut i = indentation.to_owned();
                    i.push_str("  ");
                    dive_in(next_pair, &i);
                } else {
                    println!("{indentation}- {:?}: {:?}", pair.as_rule(), pair.as_str());
                }
            }
        }

        dive_in(pairs, "");
    }

    #[test]
    fn test_get_request_example_with_rule_matching() {
        let input = include_str!("../get_example.http");
        let mut pairs = RequestParser::parse_request(input).unwrap();

        let request = pairs.next().unwrap();
        assert_eq!(request.as_rule(), Rule::request);
        println!(
            "{:?} with this content: {:?}",
            request.as_rule(),
            request.as_str()
        );

        let mut all_request_parts = request.into_inner();
        let http_method = all_request_parts.next().unwrap();
        assert_eq!(http_method.as_rule(), Rule::method);
        println!(
            "{:?} with this content: {:?}",
            http_method.as_rule(),
            http_method.as_str()
        );

        let url = all_request_parts.next().unwrap();
        assert_eq!(url.as_rule(), Rule::url);
        println!("{:?} with this content: {:?}", url.as_rule(), url.as_str());

        let scheme = url.into_inner().next().unwrap();
        assert_eq!(scheme.as_rule(), Rule::scheme);
        println!(
            "{:?} with this content: {:?}",
            scheme.as_rule(),
            scheme.as_str()
        );
    }
}
