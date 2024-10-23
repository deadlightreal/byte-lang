#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Terminate,
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: DeclareVariableType,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclareVariableType {
    String,
    Number
}
