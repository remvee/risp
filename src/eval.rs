use parse;

fn call(oper: &parse::Node, x: i64, y: i64) -> i64 {
    let identifier = match oper {
        parse::Node::Identifier(v) => v,
        _ => panic!("Call on none identifier: {:?}", oper),
    };
    if identifier == "+" {
        x + y
    } else if identifier == "-" {
        x - y
    } else {
        panic!("Call on unknown identifier: {:?}", identifier)
    }
}

fn run(node: &parse::Node) -> i64 {
    match node {
        parse::Node::Form(nodes) => call(&nodes[0], run(&nodes[1]), run(&nodes[2])),
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
        assert_eq!(13, eval("(+ 10 (- 5 2))"))
    }
}
