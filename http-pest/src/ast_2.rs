use super::Rule;
use pest::Span;
use pest_ast::*;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::method))]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::scheme))]
pub enum Scheme {
    Http,
    Https,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::version))]
pub enum Version {
    Http1_1,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::port))]
pub struct Port(
    #[pest_ast(outer(with(span_into_str), with(str::parse::<u16>), with(Result::unwrap_or_default)))]
     u16,
);

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::url))]
pub struct Url {
    scheme: Scheme,
    #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
    host: String,
    port: Option<Port>,
    #[pest_ast(outer(with(span_into_str), with(str::to_string)))]
    path: String,
    params: Vec<Param>,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::param_name))]
pub struct ParamName(#[pest_ast(outer(with(span_into_str), with(str::to_string)))] String);

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::param_value))]
pub struct ParamValue(#[pest_ast(outer(with(span_into_str), with(str::to_string)))] String);

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::param))]
pub struct Param {
    name: ParamName,
    value: ParamValue,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::header_name))]
pub struct HeaderName(#[pest_ast(outer(with(span_into_str), with(str::to_string)))] String);

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::header_value))]
pub struct HeaderValue(#[pest_ast(outer(with(span_into_str), with(str::to_string)))] String);

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::header))]
pub struct Header {
    name: HeaderName,
    value: HeaderValue,
}

#[derive(Debug, PartialEq, Eq, FromPest)]
#[pest_ast(rule(Rule::request))]
pub struct Request {
    method: Method,
    url: Url,
    version: Version,
    headers: Vec<Header>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use from_pest::FromPest;

    #[test]
    fn test_rule_to_ast_parsing_for_method() {
        let input = include_str!("../get_example.http");
        let mut parse_tree = RequestParser::parse(Rule::method, input).unwrap();
        let ast: Method = Method::from_pest(&mut parse_tree).unwrap();

        assert_eq!(ast, Method::Get);
    }

    #[test]
    fn test_rule_to_ast_parsing_whole_request() {
        let input = include_str!("../get_example.http");
        let mut parse_tree = RequestParser::parse(Rule::request, input).unwrap();
        dbg!(&parse_tree);
        let ast: Request = Request::from_pest(&mut parse_tree).unwrap();

        assert_eq!(&ast.method, &Method::Get);
    }
}
