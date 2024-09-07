use super::arg_type::ArgType;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionStruct {
    pub name : String,
    pub content : String,
    pub args : Vec<ArgType>,
}
