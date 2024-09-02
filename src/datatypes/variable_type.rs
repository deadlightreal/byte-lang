use super::{DataNumber, DataBoolean, DataString};

// All variable types.

#[derive(Clone, PartialEq, Debug)]
pub enum VariableType {
    Number(DataNumber),
    String(DataString),
    Bool(DataBoolean),
}
