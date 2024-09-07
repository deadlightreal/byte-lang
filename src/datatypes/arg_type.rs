#[derive(Debug, Clone, PartialEq)]
pub enum ArgType {
    Bool(String),
    String(String),
    Number(String),
}
