use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Converter(HashMap<usize, usize>);

impl From<HashMap<usize, usize>> for Converter {
    fn from(m: HashMap<usize, usize>) -> Self {
        Self(m)
    }
}

impl Converter {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn conv(&self, i: usize) -> Option<usize> {
        self.0.get(&i).copied()
    }
}
