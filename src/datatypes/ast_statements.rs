#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub col: usize,
    pub line: usize,
    pub start_pos: usize,
    pub end_pos: usize,
    pub statement_type: Statements
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Terminate,
    EOF,
    VariableDeclaration(VariableDeclaration),
    BuildInFunctions(BuildInFunctionsAst),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuildInFunctionsAst {
    Println(String)
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
