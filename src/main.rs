use std::iter::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Node {
    Form(Vec<Node>),
    Identifier(String),
    Integer(i64)
}

fn parse_int(input : &str, offset : usize) -> (Node, usize) {
    let input: String = input.chars().skip(offset).take_while(|c| c.is_numeric()).collect();
    (Node::Integer(input.parse().unwrap()), offset + input.len())
}

#[test]
fn test_parse_int() {
    assert_eq!((Node::Integer(42), 2), parse_int("42", 0));
    assert_eq!((Node::Integer(2), 2), parse_int("42", 1));
}

fn parse_identifier(input : &str, offset : usize) -> (Node, usize) {
    let input: String = input.chars().skip(offset).take_while(|c| !c.is_whitespace()).collect();
    (Node::Identifier(input.to_string()), offset + input.len())
}

#[test]
fn test_parse_identifier() {
    assert_eq!((Node::Identifier("test".to_string()), 4), parse_identifier("test", 0));
    assert_eq!((Node::Identifier("+".to_string()), 1), parse_identifier("+", 0));
}

fn parse_form(input : &str, offset : usize) -> (Node, usize) {
    let mut nodes: Vec<Node> = Vec::new();
    let mut offset = offset + 1;
    loop {
        let mut iter = input.char_indices().skip(offset);
        let (i, c) = iter.next().unwrap();
        if c == ')' {
            return (Node::Form(nodes), i + 1);
        } else if c == '(' {
            let (node, j) = parse_form(input, i);
            offset = j;
            nodes.push(node);
        } else if char::is_numeric(c) {
            let (node, j) = parse_int(input, i);
            offset = j;
            nodes.push(node);
        } else if char::is_whitespace(c) {
            offset = i + 1;
        } else {
            let (node, j) = parse_identifier(input, i);
            offset = j;
            nodes.push(node);
        }
    }
}

#[test]
fn test_parse_form() {
    assert_eq!(
        (Node::Form([Node::Integer(42)].to_vec()), 4),
        parse_form("(42)", 0)
    );
    assert_eq!(
        (Node::Form([Node::Integer(42), Node::Integer(33)].to_vec()), 7),
        parse_form("(42 33)", 0)
    );
    assert_eq!(
        (Node::Form([Node::Integer(42), Node::Form([Node::Integer(9), Node::Integer(33)].to_vec())].to_vec()), 11),
        parse_form("(42 (9 33))", 0)
    );
    assert_eq!(
        (Node::Form([Node::Identifier("+".to_string()),
                     Node::Integer(42),
                     Node::Form([Node::Identifier("-".to_string()),
                                 Node::Integer(9),
                                 Node::Integer(33)].to_vec())].to_vec()), 15),
        parse_form("(+ 42 (- 9 33))", 0)
    );
}

fn call(oper : &Node, x : i64, y : i64) -> i64 {
    let identifier = match oper { Node::Identifier(v) => v, _ => "+" };
    if identifier == "+" {
        x + y
    } else if identifier == "-" {
        x - y
    } else {
        x
    }
}

fn run(node : &Node) -> i64 {
    match node {
        Node::Form(nodes) => {
            call(&nodes[0], run(&nodes[1]), run(&nodes[2]))
        }
        Node::Integer(n) => *n,
        _ => 0
    }
}

fn eval(input : &str) -> i64 {
    let (node, _) = parse_form(&input, 0);
    run(&node)
}

#[test]
fn test_eval() {
    assert_eq!(
        13,
        eval("(+ 10 (- 5 2))")
    )
}
