use super::variable_type::VariableType;

#[derive(Debug, PartialEq, Clone)]
pub struct StackItem {
    pub offset: u32,
    pub variable : VariableType,
    pub size : u32,
}
