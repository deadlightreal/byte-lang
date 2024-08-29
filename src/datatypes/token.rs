use super::print_string::PrintString;
use super::data_string::DataString;
use super::data_number::DataNumber;
use super::compare::Compare;
use super::loop_token::Loop_Token;

#[derive(Debug, PartialEq)]
pub enum Token {
    PrintVariable(String),
    PrintlnVariable(String),
    PrintString(PrintString),
    EOF,
    Error(String),
    String(DataString),
    Comment,
    PrintlnString(PrintString),
    WaitNumber(f64),
    Loop(Loop_Token),
    Number(DataNumber),
    Compare(Compare),
    Terminate(),
}
