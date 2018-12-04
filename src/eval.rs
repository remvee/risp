use parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    IdentifierExpected,
    FormOrValueExpected,
    UnknownFunction,
    ParseError { err: parse::ParseError },
}

type EvalValue = i64;
type EvalResult = Result<EvalValue, EvalError>;

struct State {
    fns: HashMap<String, fn(&[parse::Node], &State) -> EvalResult>,
}

fn call(nodes: &Vec<parse::Node>, state: &State) -> EvalResult {
    let oper = &nodes[0];
    let identifier = match oper {
        parse::Node::Identifier(v) => v,
        _ => return Err(EvalError::IdentifierExpected),
    };

    let func = match state.fns.get(identifier) {
        Some(f) => f,
        None => return Err(EvalError::UnknownFunction),
    };

    func(&nodes[1..], &state)
}

fn run(node: &parse::Node, state: &State) -> EvalResult {
    match node {
        parse::Node::Form(nodes) => call(&nodes, state),
        parse::Node::Integer(n) => Ok(*n),
        _ => return Err(EvalError::FormOrValueExpected),
    }
}

fn fn_add(nodes: &[parse::Node], state: &State) -> EvalResult {
    let mut n = 0;
    for node in &nodes[0..] {
        n += run(node, state)?;
    }
    Ok(n)
}

fn fn_subtract(nodes: &[parse::Node], state: &State) -> EvalResult {
    if nodes.is_empty() {
        return Ok(0);
    }

    let mut n = run(&nodes[0], state)?;
    for node in &nodes[1..] {
        n -= run(node, state)?;
    }
    Ok(n)
}

fn initial_state() -> State {
    let mut state: State = State {
        fns: HashMap::new(),
    };
    state.fns.insert("+".to_string(), fn_add);
    state.fns.insert("-".to_string(), fn_subtract);
    state
}

pub fn eval(input: &str) -> EvalResult {
    let mut result = Ok(0);
    let state = initial_state();
    let nodes = match parse::parse(&input) {
        Ok(x) => x,
        Err(err) => return Err(EvalError::ParseError { err: err }),
    };
    for node in nodes {
        result = run(&node, &state)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(Ok(10), eval("(+ 10)"));
        assert_eq!(Ok(20), eval("(+ 10 (- 5 2 3) 1 9)"));
        assert_eq!(Ok(1), eval("1"));
        assert_eq!(Ok(6), eval("1 (+ 2) (+ 1 2 3)"));
        assert_eq!(Ok(0), eval("(+)"));
        assert_eq!(Ok(0), eval("(-)"));
    }
    #[test]
    fn test_eval_errors() {
        assert_eq!(
            Err(EvalError::UnknownFunction),
            eval("(not-a-function 1 2)")
        );
        assert_eq!(Err(EvalError::IdentifierExpected), eval("(1 2)"));
        assert_eq!(Err(EvalError::FormOrValueExpected), eval("not-a-value"));
        assert_eq!(
            Err(EvalError::ParseError {
                err: parse::ParseError::UnexpectedEndOfInput { at: 1 }
            }),
            eval("(")
        );
        assert_eq!(
            Err(EvalError::ParseError {
                err: parse::ParseError::NoIdentifier { at: 0 }
            }),
            eval(")")
        );
        assert_eq!(
            Err(EvalError::ParseError {
                err: parse::ParseError::FailedToParseInteger { at: 5 }
            }),
            eval("(+ 1 2x)")
        );
    }
}
