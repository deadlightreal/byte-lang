use super::{Statement, Statements, BuildInFunctionsAst, StackItem, StackFrame, StaticData, Expression, Literal};
use std::collections::HashMap;

pub struct CodeGenerator<'a> {
    input: &'a Vec<Statement>,
    position: usize,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(input: &'a Vec<Statement>) -> Self {
        return Self{input, position: 0};
    }

    pub fn generate_all(&mut self) -> Vec<String> {
        let mut stack : Vec<HashMap<String, StackItem>> = vec![HashMap::new()];
        let mut static_data : Vec<StaticData> = Vec::new();

        let mut res : Vec<String> = Vec::new();

        loop {
            match self.generate_next(&mut stack, &mut static_data) {
                Some(str_res) => {
                    res.push(str_res);
                },
                None => {
                    break;
                },
            }
        };

        res.push(String::from(".data\n"));
        
        for (i, v) in static_data.iter().enumerate() {
            match v {
                StaticData::PrintString(str) => {
                    res.push(format!("static_{}: .ascii \"{}\"\n", i, str));
                },
            };
        }

        return res;
    }

    pub fn generate_next(&mut self, stack : &mut Vec<HashMap<String, StackItem>>, static_data : &mut Vec<StaticData>) -> Option<String> {
        let current = self.current_statement();
        self.position += 1;

        match current.unwrap().statement_type {
            Statements::EOF => {
                return None;
            },
            Statements::Terminate => {
                return Some(String::from("mov x0, #0\nmov x16, #1\nsvc #0X80\n"));
            },
            Statements::VariableDeclaration(var) => {
                println!("{:?}", var);

                let value = match var.value {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::Number(num) => num,
                            Literal::String(_) => 10
                        }
                    },
                };

                return Some(String::from(&format!("mov x0, #{}\nstr x0, [sp]\nsub sp, sp, #16\n", value)));
            },
            Statements::BuildInFunctions(func) => {
                match func {
                    BuildInFunctionsAst::Println(string) => {
                        let mut string = string.clone();
                        string.push_str("\\n");

                        static_data.push(StaticData::PrintString(string.clone()));

                        let bytes_in_string = string.clone().replace("\\n", "\n").len();

                        return Some(String::from(format!("mov x0, #1\nadrp x1, static_{}@PAGE\nadd x1, x1, static_{}@PAGEOFF\nmov x2, #{}\nmov x16, #4\nsvc 0x80\n", static_data.len() - 1, static_data.len() - 1, bytes_in_string)))
                    }
                };
            },
            _ => ()
        };

        return None;
    }

    pub fn current_statement(&mut self) -> Option<Statement> {
        match self.input.get(self.position) {
            Some(statement) => {return Some(statement.clone())},
            None => {return None}
        };
    }
}
