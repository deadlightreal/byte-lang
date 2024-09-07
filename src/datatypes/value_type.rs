#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    String(String),
    Number(i64),
    Boolean(bool),
}
