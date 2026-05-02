#[derive(Clone, Copy)]
pub struct ParseOptions {
    pub max_depth: Option<usize>,
}

impl ParseOptions {
    pub fn unlimited() -> Self {
        Self { max_depth: None }
    }
}
