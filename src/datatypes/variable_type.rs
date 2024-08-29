use super::{DataNumber, DataString};

// All variable types.

#[derive(Clone)]
pub enum VariableType {
    Number(DataNumber),
    String(DataString)
}
