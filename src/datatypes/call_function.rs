use super::FunctionStruct;
use super::FunctionArg;

#[derive(Clone, PartialEq, Debug)]
pub struct CallFunction {
    pub function: FunctionStruct,
    pub args: Vec<FunctionArg>,
}
