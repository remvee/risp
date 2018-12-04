use parse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum EvalError {
    IdentifierExpected { at: usize },
    FormOrValueExpected { at: usize },
    NotAnInteger { at: usize },
    UnknownFunction { at: usize, name: String },
    ParseError { err: parse::ParseError },
    EmptyForm,
}

#[derive(Debug, PartialEq)]
pub enum EvalValue {
    Integer(i64),
    String(String),
}

type EvalResult = Result<EvalValue, EvalError>;

struct State {
    fns: HashMap<String, fn(&[parse::Node], &State) -> EvalResult>,
}

fn call(nodes: &Vec<parse::Node>, state: &State) -> EvalResult {
    if nodes.is_empty() {
        return Err(EvalError::EmptyForm);
    }

    let oper = &nodes[0];
    let identifier = match oper {
        parse::Node::Identifier { payload, at: _ } => payload,
        _ => {
            return Err(EvalError::IdentifierExpected {
                at: parse::at(oper),
            })
        }
    };

    let func = match state.fns.get(identifier) {
        Some(f) => f,
        None => {
            return Err(EvalError::UnknownFunction {
                at: parse::at(oper),
                name: identifier.to_string(),
            })
        }
    };

    func(&nodes[1..], &state)
}

fn run(node: &parse::Node, state: &State) -> EvalResult {
    match node {
        parse::Node::Form { payload, at: _ } => call(&payload, state),
        parse::Node::Integer { payload, at: _ } => Ok(EvalValue::Integer(*payload)),
        parse::Node::String { payload, at: _ } => Ok(EvalValue::String(payload.to_string())),
        _ => {
            return Err(EvalError::FormOrValueExpected {
                at: parse::at(node),
            })
        }
    }
}

fn fn_add(nodes: &[parse::Node], state: &State) -> EvalResult {
    let mut n = 0;
    for node in &nodes[0..] {
        n += match run(node, state)? {
            EvalValue::Integer(m) => m,
            _ => {
                return Err(EvalError::NotAnInteger {
                    at: parse::at(node),
                })
            }
        };
    }
    Ok(EvalValue::Integer(n))
}

fn fn_subtract(nodes: &[parse::Node], state: &State) -> EvalResult {
    if nodes.is_empty() {
        return Ok(EvalValue::Integer(0));
    }

    let mut n = match run(&nodes[0], state)? {
        EvalValue::Integer(m) => m,
        _ => {
            return Err(EvalError::NotAnInteger {
                at: parse::at(&nodes[0]),
            })
        }
    };
    for node in &nodes[1..] {
        n -= match run(node, state)? {
            EvalValue::Integer(m) => m,
            _ => {
                return Err(EvalError::NotAnInteger {
                    at: parse::at(node),
                })
            }
        };
    }
    Ok(EvalValue::Integer(n))
}

fn fn_str(nodes: &[parse::Node], state: &State) -> EvalResult {
    let mut r = "".to_owned();
    if nodes.is_empty() {
        return Ok(EvalValue::String(r));
    }
    for node in &nodes[0..] {
        r += &match run(node, state)? {
            EvalValue::Integer(v) => v.to_string(),
            EvalValue::String(v) => v,
        };
    }
    Ok(EvalValue::String(r))
}

fn initial_state() -> State {
    let mut state: State = State {
        fns: HashMap::new(),
    };
    state.fns.insert("+".to_string(), fn_add);
    state.fns.insert("-".to_string(), fn_subtract);
    state.fns.insert("str".to_string(), fn_str);
    state
}

pub fn eval(input: &str) -> EvalResult {
    let mut result = Ok(EvalValue::Integer(0));
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
        assert_eq!(Ok(EvalValue::Integer(10)), eval("(+ 10)"));
        assert_eq!(Ok(EvalValue::Integer(2)), eval("(- 5 2 1)"));
        assert_eq!(Ok(EvalValue::Integer(20)), eval("(+ 10 (- 5 2 3) 1 9)"));
        assert_eq!(Ok(EvalValue::Integer(1)), eval("1"));
        assert_eq!(Ok(EvalValue::Integer(6)), eval("1 (+ 2) (+ 1 2 3)"));
        assert_eq!(Ok(EvalValue::Integer(0)), eval("(+)"));
        assert_eq!(Ok(EvalValue::Integer(0)), eval("(-)"));
        assert_eq!(
            Ok(EvalValue::String("123go!".to_string())),
            eval("(str 1 2 3 \"go!\")")
        );
    }
    #[test]
    fn test_eval_errors() {
        assert_eq!(
            Err(EvalError::UnknownFunction {
                at: 1,
                name: "not-a-function".to_string()
            }),
            eval("(not-a-function 1 2)")
        );
        assert_eq!(Err(EvalError::IdentifierExpected { at: 1 }), eval("(1 2)"));
        assert_eq!(
            Err(EvalError::FormOrValueExpected { at: 0 }),
            eval("not-a-value")
        );
        assert_eq!(
            Err(EvalError::ParseError {
                err: parse::ParseError::FailedToParseInteger { at: 5 }
            }),
            eval("(+ 1 2x)")
        );
        assert_eq!(
            Err(EvalError::NotAnInteger { at: 5 }),
            eval("(+ 1 (str 1 2 3))")
        );
        assert_eq!(Err(EvalError::EmptyForm), eval("()"));
    }
}
