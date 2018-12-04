use std::iter::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    Form { payload: Vec<Node>, at: usize },
    Identifier { payload: String, at: usize },
    Integer { payload: i64, at: usize },
    String { payload: String, at: usize },
}

pub fn at(node: &Node) -> usize {
    match node {
        Node::Form { at, .. }
        | Node::Identifier { at, .. }
        | Node::Integer { at, .. }
        | Node::String { at, .. } => *at,
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEndOfInput { at: usize },
    FailedToParseInteger { at: usize },
    NoIdentifier { at: usize },
    UnexpectedInput { at: usize },
    UnbalancedParentheses { at: usize },
}

pub type ParseResult = Result<Vec<Node>, ParseError>;

pub fn parse(input: &str) -> ParseResult {
    return match parse_form(input, 0, true) {
        Ok((node, ..)) => match node {
            Node::Form { payload, .. } => Ok(payload),
            _ => Err(ParseError::UnexpectedInput { at: at(&node) }),
        },
        Err(err) => Err(err),
    };
}

type ParseInternalResult = Result<(Node, usize), ParseError>;

fn parse_int(input: &str, offset: usize) -> ParseInternalResult {
    let input: String = input
        .chars()
        .skip(offset)
        .take_while(|c| !c.is_whitespace() && c != &')')
        .collect();
    match input.parse() {
        Ok(n) => Ok((
            Node::Integer {
                payload: n,
                at: offset,
            },
            offset + input.len(),
        )),
        Err(_) => Err(ParseError::FailedToParseInteger { at: offset }),
    }
}

fn parse_str(input: &str, offset: usize) -> ParseInternalResult {
    let input: String = input
        .chars()
        .skip(offset + 1)
        .take_while(|c| c != &'"')
        .collect();
    Ok((
        Node::String {
            payload: input.to_string(),
            at: offset,
        },
        offset + input.len() + 2,
    ))
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
        Ok((
            Node::Identifier {
                payload: input.to_string(),
                at: offset,
            },
            offset + input.len(),
        ))
    }
}

fn parse_form(input: &str, offset: usize, outer: bool) -> ParseInternalResult {
    let mut nodes: Vec<Node> = Vec::new();
    let mut i = offset;
    loop {
        let mut iter = input
            .char_indices()
            .skip(i)
            .skip_while(|(_, c)| c.is_whitespace());
        let (j, c) = match iter.next() {
            Some((j, c)) => (j, c),
            None => {
                if outer {
                    return Ok((
                        Node::Form {
                            payload: nodes,
                            at: offset,
                        },
                        i + 1,
                    ));
                } else {
                    return Err(ParseError::UnexpectedEndOfInput { at: offset });
                }
            }
        };
        if c == ')' {
            if outer {
                return Err(ParseError::UnbalancedParentheses { at: j });
            } else {
                return Ok((
                    Node::Form {
                        payload: nodes,
                        at: offset - 1,
                    },
                    j + 1,
                ));
            }
        } else if c == '(' {
            let (node, k) = parse_form(input, j + 1, false)?;
            i = k;
            nodes.push(node);
        } else if char::is_numeric(c) {
            let (node, k) = parse_int(input, j)?;
            i = k;
            nodes.push(node);
        } else if c == '"' {
            let (node, k) = parse_str(input, i + 1)?;
            i = k;
            nodes.push(node);
        } else if char::is_whitespace(c) {
            i = j + 1;
        } else {
            let (node, k) = parse_identifier(input, j)?;
            i = k;
            nodes.push(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(
            Ok((Node::Integer { payload: 42, at: 0 }, 2)),
            parse_int("42", 0)
        );
        assert_eq!(
            Ok((Node::Integer { payload: 2, at: 1 }, 2)),
            parse_int("42", 1)
        );
        assert_eq!(
            Err(ParseError::FailedToParseInteger { at: 1 }),
            parse_int(" 1x", 1)
        );
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(
            Ok((
                Node::String {
                    payload: "foo".to_string(),
                    at: 0
                },
                5
            )),
            parse_str("\"foo\"", 0)
        );
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(
            Ok((
                Node::Identifier {
                    payload: "test".to_string(),
                    at: 0
                },
                4
            )),
            parse_identifier("test", 0)
        );
        assert_eq!(
            Ok((
                Node::Identifier {
                    payload: "+".to_string(),
                    at: 0
                },
                1
            )),
            parse_identifier("+", 0)
        );
        assert_eq!(
            Err(ParseError::NoIdentifier { at: 1 }),
            parse_identifier("f ", 1)
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(Ok(vec![Node::Integer { payload: 42, at: 0 }]), parse("42"));
        assert_eq!(
            Ok(vec![Node::Form {
                payload: vec![Node::Integer { payload: 42, at: 1 }],
                at: 0
            }]),
            parse("(42)")
        );
        assert_eq!(
            Ok(vec![Node::Form {
                payload: vec![
                    Node::Integer { payload: 42, at: 1 },
                    Node::Integer { payload: 33, at: 4 }
                ],
                at: 0
            }]),
            parse("(42 33)")
        );
        assert_eq!(
            Ok(vec![Node::Form {
                payload: vec![
                    Node::Integer { payload: 42, at: 1 },
                    Node::Form {
                        payload: vec![
                            Node::Integer { payload: 9, at: 5 },
                            Node::Integer { payload: 33, at: 7 }
                        ],
                        at: 4
                    }
                ],
                at: 0
            }]),
            parse("(42 (9 33))")
        );
        assert_eq!(
            Ok(vec![Node::Form {
                payload: vec![
                    Node::Identifier {
                        payload: "+".to_string(),
                        at: 1
                    },
                    Node::Integer { payload: 42, at: 3 },
                    Node::Form {
                        payload: vec![
                            Node::Identifier {
                                payload: "-".to_string(),
                                at: 7
                            },
                            Node::Integer { payload: 9, at: 9 },
                            Node::Integer {
                                payload: 33,
                                at: 11
                            }
                        ],
                        at: 6
                    }
                ],
                at: 0
            }]),
            parse("(+ 42 (- 9 33))")
        );
        assert_eq!(
            Ok(vec![Node::Form {
                payload: vec![
                    Node::Identifier {
                        payload: "def".to_string(),
                        at: 1
                    },
                    Node::Identifier {
                        payload: "inc".to_string(),
                        at: 5
                    },
                    Node::Form {
                        payload: vec![Node::Identifier {
                            payload: "x".to_string(),
                            at: 10
                        }],
                        at: 9
                    },
                    Node::Form {
                        payload: vec![
                            Node::Identifier {
                                payload: "+".to_string(),
                                at: 14
                            },
                            Node::Integer { payload: 1, at: 16 },
                            Node::Identifier {
                                payload: "x".to_string(),
                                at: 18
                            }
                        ],
                        at: 13
                    },
                ],
                at: 0
            }]),
            parse("(def inc (x) (+ 1 x))")
        );
        assert_eq!(Err(ParseError::UnexpectedEndOfInput { at: 1 }), parse("("));
        assert_eq!(Err(ParseError::UnbalancedParentheses { at: 0 }), parse(")"));
    }
}
