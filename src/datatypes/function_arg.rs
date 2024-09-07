use super::stack_item::StackItem;
use super::value_type::ValueType;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArg {
    Variable(StackItem),
    Value(ValueType),
}
