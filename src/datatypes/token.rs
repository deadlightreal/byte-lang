use super::print_string::PrintString;
use super::data_string::DataString;
use super::data_number::DataNumber;
use super::compare::Compare;
use super::loop_token::LoopToken;
use super::function_struct::FunctionStruct;
use super::data_boolean::DataBoolean;

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
    Loop(LoopToken),
    Number(DataNumber),
    Compare(Compare),
    Terminate(),
    Function(FunctionStruct),
    CallFunction(String),
    DataBoolean(DataBoolean),
}
