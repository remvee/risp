use std::iter::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    Form(Vec<Node>),
    Identifier(String),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEndOfInput { at: usize },
    FailedToParseInteger { at: usize },
    NoIdentifier { at: usize },
}

pub type ParseResult = Result<Vec<Node>, ParseError>;

pub fn parse(input: &str) -> ParseResult {
    let mut nodes: Vec<Node> = Vec::new();
    let mut offset = 0;
    loop {
        let mut iter = input
            .char_indices()
            .skip(offset)
            .skip_while(|(_, c)| c.is_whitespace());
        let (i, c) = match iter.next() {
            Some((i, c)) => (i, c),
            None => return Ok(nodes),
        };
        if c == '(' {
            let (node, j) = parse_form(input, i)?;
            offset = j;
            nodes.push(node);
        } else if char::is_numeric(c) {
            let (node, j) = parse_int(input, i)?;
            offset = j;
            nodes.push(node);
        } else if char::is_whitespace(c) {
            offset = i + 1;
        } else {
            let (node, j) = parse_identifier(input, i)?;
            offset = j;
            nodes.push(node);
        }
    }
}

type ParseInternalResult = Result<(Node, usize), ParseError>;

fn parse_int(input: &str, offset: usize) -> ParseInternalResult {
    let input: String = input
        .chars()
        .skip(offset)
        .take_while(|c| !c.is_whitespace() && c != &')')
        .collect();
    match input.parse() {
        Ok(n) => Ok((Node::Integer(n), offset + input.len())),
        Err(_) => Err(ParseError::FailedToParseInteger { at: offset }),
    }
}

fn parse_identifier(input: &str, offset: usize) -> ParseInternalResult {
    let input: String = input
        .chars()
        .skip(offset)
        .take_while(|c| !(c.is_whitespace() || c == &')'))
        .collect();
    if input.is_empty() {
        Err(ParseError::NoIdentifier { at: offset })
    } else {
        Ok((Node::Identifier(input.to_string()), offset + input.len()))
    }
}

fn parse_form(input: &str, offset: usize) -> ParseInternalResult {
    let mut nodes: Vec<Node> = Vec::new();
    let mut offset = offset + 1;
    loop {
        let mut iter = input.char_indices().skip(offset);
        let (i, c) = match iter.next() {
            Some((i, c)) => (i, c),
            None => return Err(ParseError::UnexpectedEndOfInput { at: offset }),
        };
        if c == ')' {
            return Ok((Node::Form(nodes), i + 1));
        } else if c == '(' {
            let (node, j) = parse_form(input, i)?;
            offset = j;
            nodes.push(node);
        } else if char::is_numeric(c) {
            let (node, j) = parse_int(input, i)?;
            offset = j;
            nodes.push(node);
        } else if char::is_whitespace(c) {
            offset = i + 1;
        } else {
            let (node, j) = parse_identifier(input, i)?;
            offset = j;
            nodes.push(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(Ok((Node::Integer(42), 2)), parse_int("42", 0));
        assert_eq!(Ok((Node::Integer(2), 2)), parse_int("42", 1));
        assert_eq!(
            Err(ParseError::FailedToParseInteger { at: 1 }),
            parse_int(" 1x", 1)
        );
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            Ok((Node::Identifier("test".to_string()), 4)),
            parse_identifier("test", 0)
        );
        assert_eq!(
            Ok((Node::Identifier("+".to_string()), 1)),
            parse_identifier("+", 0)
        );
        assert_eq!(
            Err(ParseError::NoIdentifier { at: 1 }),
            parse_identifier("f ", 1)
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(Ok(vec![Node::Form(vec![Node::Integer(42)])]), parse("(42)"));
        assert_eq!(
            Ok(vec![Node::Form(vec![Node::Integer(42), Node::Integer(33)])]),
            parse("(42 33)")
        );
        assert_eq!(
            Ok(vec![Node::Form(vec![
                Node::Integer(42),
                Node::Form(vec![Node::Integer(9), Node::Integer(33)])
            ])]),
            parse("(42 (9 33))")
        );
        assert_eq!(
            Ok(vec![Node::Form(vec![
                Node::Identifier("+".to_string()),
                Node::Integer(42),
                Node::Form(vec![
                    Node::Identifier("-".to_string()),
                    Node::Integer(9),
                    Node::Integer(33)
                ])
            ])]),
            parse("(+ 42 (- 9 33))")
        );
        assert_eq!(
            Ok(vec![Node::Form(vec![
                Node::Identifier("def".to_string()),
                Node::Identifier("inc".to_string()),
                Node::Form(vec![Node::Identifier("x".to_string())]),
                Node::Form(vec![
                    Node::Identifier("+".to_string()),
                    Node::Integer(1),
                    Node::Identifier("x".to_string())
                ]),
            ])]),
            parse("(def inc (x) (+ 1 x))")
        );
    }
}
