use std::io;
//use std::io::Read;
use std::iter::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Node {
    Form(Vec<Node>),
    String(String),
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

fn parse(input : &str, offset : usize) -> (Node, usize) {
    let input = input.trim();
    let c = input.chars().next().unwrap();

    if c == '(' {
        parse_form(input, offset)
    } else if char::is_numeric(c) {
        parse_int(input, offset)
    } else if c == '"' {
        (Node::String(String::from(input)), 0)
    } else {
        (Node::Identifier(String::from(input)), 0)
    }

    // match iter.next() {
    //     Some(x) => {
    //         println!("is digit: {:?}", char::is_numeric(x));
    //     }
    //     None => {
    //         println!("no more input?");
    //     }
    // }
    // let form = form.trim();

    // if form.starts_with('(') {
    //     Node::Form(vec![parse(&form[1 .. form.len() - 1])])
    // } else if form.starts_with('"') {
    //     Node::String(form.to_string())
    // } else if starts_with_digit(form) {
    //     Node::Integer(31415)
    // } else {
    //     Node::Identifier(form.to_string())
    // }
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    // let mut code = String::new();
    // io::stdin().read_to_string(&mut code)?;
    let code = String::from("(+ 1123 (- 34 52))");
    println!("DONE {:?}", parse(&code, 0));

    Ok(())
}
