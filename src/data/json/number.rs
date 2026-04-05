#[derive(PartialEq, Eq, Debug)]
pub struct Number {
    pub integer: i64,
    pub fraction: Option<u64>,
    pub exponent: Option<i64>,
}
