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

fn to_funcs(token: &str) -> Option<StackFunc> {
    if let Ok(num) = token.parse::<i64>() {
        return closure!(move |_x, stack: &mut Stack| {
            stack.push(num as f64);
            Some(())
        });
    };
    match token {
        "+" => closure!(|_x, stack: &mut Stack| {
            let sum = stack.pop()? + stack.pop()?;
            stack.push(sum);
            Some(())
        }),
        "-" => closure!(|_x, stack: &mut Stack| {
            let not_sum = stack.pop()? - stack.pop()?;
            stack.push(not_sum);
            Some(())
        }),
        "*" => closure!(|_x, stack: &mut Stack| {
            let product = stack.pop()? * stack.pop()?;
            stack.push(product);
            Some(())
        }),
        "/" => closure!(|_x, stack: &mut Stack| {
            let quotient = stack.pop()? / stack.pop()?;
            stack.push(quotient);
            Some(())
        }),
        "x" => closure!(|x: f64, stack: &mut Stack| {
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
