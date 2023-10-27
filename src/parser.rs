struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Self { stack: Vec::new() };
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

pub fn parse_equation(equation: &str) -> Box<dyn Fn(f64) -> Option<f64>> {
    let actual = equation.to_owned();
    let func = move |x: f64| {
        let tokens = actual.split_whitespace();
        let mut stack = Stack::new();
        for token in tokens.clone() {
            if let Ok(num) = token.parse::<i64>() {
                stack.push(num as f64);
            };
            match token {
                "+" => {
                    let sum = stack.pop()? + stack.pop()?;
                    stack.push(sum);
                }
                "-" => {
                    let not_sum = stack.pop()? - stack.pop()?;
                    stack.push(not_sum);
                }
                "*" => {
                    let product = stack.pop()? * stack.pop()?;
                    stack.push(product);
                }
                "/" => {
                    let quotient = stack.pop()? / stack.pop()?;
                    stack.push(quotient);
                }
                "x" => stack.push(x),
                _ => return None,
            }
        }
        return stack.pop();
    };
    Box::new(func)
}

#[test]
fn monadic() {
    let result = parse_equation("2 x +");
    assert_eq!(result(1.0), Some(3.0));
}
