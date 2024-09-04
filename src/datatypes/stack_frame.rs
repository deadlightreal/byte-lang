use super::stack_item::StackItem;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame {
    pub stack_items: HashMap<String, StackItem>,
}
