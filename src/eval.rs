use parse;
use std::collections::HashMap;

type Result = i64;

struct State {
    fns: HashMap<String, fn(&[parse::Node], &State) -> Result>,
}

fn call(nodes: &Vec<parse::Node>, state: &State) -> Result {
    let oper = &nodes[0];
    let identifier = match oper {
        parse::Node::Identifier(v) => v,
        _ => panic!("Call on none identifier: {:?}", oper),
    };

    let func = match state.fns.get(identifier) {
        Some(f) => f,
        None => panic!("No such function {:?}", identifier),
    };

    func(&nodes[1..], &state)
}

fn run(node: &parse::Node, state: &State) -> Result {
    match node {
        parse::Node::Form(nodes) => call(&nodes, state),
        parse::Node::Integer(n) => *n,
        _ => panic!("Can not run: {:?}", node),
    }
}

fn fn_add(nodes: &[parse::Node], state: &State) -> Result {
    let mut n: Result = run(&nodes[0], state);
    for node in &nodes[1..] {
        n += run(node, state);
    }
    n
}

fn fn_subtract(nodes: &[parse::Node], state: &State) -> Result {
    let mut n: Result = run(&nodes[0], state);
    for node in &nodes[1..] {
        n -= run(node, state);
    }
    n
}

fn initial_state() -> State {
    let mut state: State = State {
        fns: HashMap::new(),
    };
    state.fns.insert("+".to_string(), fn_add);
    state.fns.insert("-".to_string(), fn_subtract);
    state
}

pub fn eval(input: &str) -> Result {
    run(&parse::parse(&input), &initial_state())
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
