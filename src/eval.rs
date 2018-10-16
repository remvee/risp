use parse;

fn call(nodes : &Vec<parse::Node>) -> i64 {
    let oper = &nodes[0];
    let identifier = match oper {
        parse::Node::Identifier(v) => v,
        _ => panic!("Call on none identifier: {:?}", oper),
    };
    if identifier == "+" {
        let mut n : i64 = run(&nodes[1]);
        for node in &nodes[2..] { n += run(node); }
        n
    } else if identifier == "-" {
        let mut n : i64 = run(&nodes[1]);
        for node in &nodes[2..] { n -= run(node); }
        n
    } else {
        panic!("Call on unknown identifier: {:?}", identifier)
    }
}

fn run(node: &parse::Node) -> i64 {
    match node {
        parse::Node::Form(nodes) => call(&nodes),
        parse::Node::Integer(n) => *n,
        _ => panic!("Can not run: {:?}", node),
    }
}

pub fn eval(input: &str) -> i64 {
    run(&parse::parse(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(10, eval("(+ 10)"));
        assert_eq!(20, eval("(+ 10 (- 5 2 3) 1 9)"));
    }
}
