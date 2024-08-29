use super::DataNumber;

// Types that can be used to compare.

#[derive(Clone, Debug, PartialEq)]
pub enum CompareType {
    Number(i64),
    VariableNumber(DataNumber),
    None()
}
