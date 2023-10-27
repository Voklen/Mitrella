use crate::parser::parse_equation;

#[derive(Default)]
pub struct Equations {
    pub strings: Vec<String>,
    pub functions: Vec<Option<Box<dyn Fn(f64) -> f64>>>,
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
