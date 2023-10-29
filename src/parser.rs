use std::f64::consts;

struct Stack {
    stack: Vec<f64>,
}

impl Stack {
    pub fn new() -> Self {
        return Self { stack: Vec::new() };
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }
}

type StackFunc = Box<dyn Fn(f64, &mut Stack) -> Option<()>>;

macro_rules! closure {
    ($input: expr) => {
        Some(Box::new($input))
    };
}

macro_rules! monadic {
    ($input: expr) => {
        closure!(|_x, stack: &mut Stack| {
            let top = stack.pop()?;
            let output = $input(top);
            stack.push(output);
            Some(())
        })
    };
}

macro_rules! dyadic {
    ($input: expr) => {
        closure!(|_x, stack: &mut Stack| {
            let top = stack.pop()?;
            let second = stack.pop()?;
            let output = $input(top, second);
            stack.push(output);
            Some(())
        })
    };
}

macro_rules! push {
    ($input: expr) => {
        closure!(|_x, stack: &mut Stack| {
            stack.push($input);
            Some(())
        })
    };
}

pub fn parse_equation(equation: &str) -> Box<dyn Fn(f64) -> Option<f64>> {
    let actual = equation.to_owned();
    let tokens = actual.split_whitespace();
    let funcs: Vec<StackFunc> = tokens.map(to_funcs).flatten().collect();
    let res = move |x: f64| run_stack_funcs(x, &funcs);
    Box::new(res)
}

fn run_stack_funcs(x: f64, funcs: &Vec<StackFunc>) -> Option<f64> {
    let mut stack = Stack::new();
    for func in funcs {
        func(x, &mut stack);
    }
    return stack.pop();
}

/// This parses a token and returns a function that modifies the stack as that token would
fn to_funcs(token: &str) -> Option<StackFunc> {
    if let Ok(num) = token.parse::<i64>() {
        return closure!(move |_x, stack: &mut Stack| {
            stack.push(num as f64);
            Some(())
        });
    };
    match token {
        // Operators
        "+" => dyadic!(|top, second| second + top),
        "-" => dyadic!(|top, second| second - top),
        "*" => dyadic!(|top, second| second * top),
        "/" => dyadic!(|top, second| second / top),
        "◿" => dyadic!(|top, second: f64| second.powf(top)),
        "^" => dyadic!(|top, second: f64| second.powf(top)),
        "◺" => dyadic!(|top: f64, second| top.powf(1.0 / second)),
        "root" => dyadic!(|top: f64, second| top.powf(1.0 / second)),
        "▽" => dyadic!(|top: f64, second| top.log(second)),
        "log" => dyadic!(|top: f64, second| top.log(second)),
        "abs" => monadic!(|top: f64| top.abs()),
        "sin" => monadic!(|top: f64| top.sin()),
        "cos" => monadic!(|top: f64| top.cos()),
        "tan" => monadic!(|top: f64| top.tan()),
        "asin" => monadic!(|top: f64| top.asin()),
        "acos" => monadic!(|top: f64| top.acos()),
        "atan" => monadic!(|top: f64| top.atan()),
        "sinh" => monadic!(|top: f64| top.sinh()),
        "cosh" => monadic!(|top: f64| top.cosh()),
        "tanh" => monadic!(|top: f64| top.tanh()),
        "asinh" => monadic!(|top: f64| top.asinh()),
        "acosh" => monadic!(|top: f64| top.acosh()),
        "atanh" => monadic!(|top: f64| top.atanh()),

        // Constants
        "e" => push!(consts::E),
        "π" => push!(consts::PI),
        "pi" => push!(consts::PI),
        "τ" => push!(consts::TAU),
        "tau" => push!(consts::TAU),

        // Miscellaneous
        "x" => closure!(|x, stack: &mut Stack| {
            stack.push(x);
            Some(())
        }),
        _ => return None,
    }
}

#[test]
fn monadic() {
    let result = parse_equation("2 x +");
    assert_eq!(result(1.0), Some(3.0));
}
