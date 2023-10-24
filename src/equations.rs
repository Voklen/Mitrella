#[derive(Default)]
pub struct Equations {
    pub strings: Vec<String>,
    pub functions: Vec<Option<fn(f64) -> f64>>,
}

impl Equations {
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    pub fn push(&mut self, string: String) {
        self.strings.push(string);
        self.functions.push(None);
    }

    pub fn update_func(&mut self, index: usize) {
        let string = &self.strings[index];
        self.functions[index] = parse_equation(string);
    }
}

fn parse_equation(equation: &str) -> Option<fn(f64) -> f64> {
    //TODO Parse equation
    match equation {
        "sin(x)" => Some(|x: f64| x.sin()),
        "cos(x)" => Some(|x: f64| x.cos()),
        _ => None,
    }
}
