use std::collections::HashMap;
use std::{fs::File, io::Read};

use super::{CompareType, FunctionStruct, Punctuations, CodeGenerator, Statement, SemanticAnaytis, StackItem, BuildInCommand, TokenType, LoopStruct, Token, Tokenizer, VariableType, StackFrame, DataNumber, ArgType, FunctionArg, ValueType, Statements, Identifiers, Literal, VariableDeclaration, BuildInFunctionsAst, Expression, DeclareVariableType, BuildInFunctions, Keywords, Operators};

pub struct Parser<'a> {
    input: &'a Vec<Token>,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<Token>) -> Self {
        return Self{input, position: 0};
    }

    pub fn parse_all(&mut self) -> Vec<Statement> {
        let mut statements : Vec<Statement> = Vec::new();

        loop {
            match self.parse_next() {
                Ok(statement) => {
                    statements.push(statement.clone());
                    if statement.statement_type == Statements::EOF {
                        break;
                    }
                },
                Err(err) => {
                    println!("{}", err);
                    break;
                },
            };
        }

        return statements;
    }

    pub fn parse_next(&mut self) -> Result<Statement, String> {
        let token = self.current_token();

        println!("{:?}", token.kind);

        let statement_default : Statement = Statement{col: token.col, line: token.line, end_pos: 0, start_pos: token.start_pos, statement_type: Statements::Terminate};

        match token.kind {
            TokenType::EOF => {
                return Ok(Statement{statement_type: Statements::EOF, end_pos: token.end_pos, ..statement_default});
            },
            TokenType::BuildInCommand(command) => {
                match command {
                    BuildInCommand::Terminate => {
                        let semicolon_token = self.current_token();
                        if semicolon_token.kind != TokenType::Semicolon {
                            return Err(String::from("Expected Semicolon after Terminate"));
                        };

                        return Ok(Statement{statement_type: Statements::Terminate, end_pos: semicolon_token.end_pos, ..statement_default});
                    }
                };
            },
            TokenType::BuildInFunctions(func) => {
                match func {
                    BuildInFunctions::Println => {
                        if self.current_token().kind != TokenType::Punctuation(Punctuations::OpenParenthesis) {
                            return Err(String::new());
                        };

                        let string_val : String = match self.current_token().kind {
                            TokenType::Identifiers(identifier) => {
                                match identifier {
                                    Identifiers::StringLiteral(str) => str,
                                    _ => {
                                        return Err(String::new());
                                    }
                                }
                            },
                            _ => {
                                return Err(String::new());
                            }
                        };
    
                        if self.current_token().kind != TokenType::Punctuation(Punctuations::ClosedParenthesis) {
                            return Err(String::new());
                        };

                        let semicolon = self.current_token();

                        if semicolon.kind != TokenType::Semicolon {
                            return Err(String::new());
                        };

                        return Ok(Statement{statement_type: Statements::BuildInFunctions(BuildInFunctionsAst::Println(string_val)), end_pos: semicolon.end_pos, ..statement_default})
                    },
                    _ => ()
               };  

                return Err(String::new());
            },
            TokenType::Keyword(keyword) => {
                let variable_type : DeclareVariableType = match keyword {
                    Keywords::NumberType => DeclareVariableType::Number,
                    Keywords::StringType => DeclareVariableType::String,
                };

                let option_name : Option<String> = match self.current_token().kind {
                    TokenType::Identifiers(identifier) => {
                        match identifier {
                            Identifiers::VariableName(name) => Some(name),
                            _ => None
                        }
                    },
                    _ => None
                };

                if option_name.is_none() {
                    return Err(String::from("Invalid Var Name"));
                };

                let name = option_name.unwrap();

                if self.current_token().kind != TokenType::Operator(Operators::Assignment) {
                    return Err(String::from("Plase Declare the variable"));
                }

                let value = self.current_token();

                let option_value : Option<Expression> = match value.kind {
                    TokenType::Identifiers(identifier) => {
                        match identifier {
                            Identifiers::StringLiteral(string_val) => Some(Expression::Literal(Literal::String(string_val))),
                            Identifiers::NumberLiteral(num_val) => Some(Expression::Literal(Literal::Number(num_val))),
                            _ => None
                        }
                    },
                    _ => None
                };

                if option_value.is_none() {
                    return Err(String::from("Please Provide a valid value"));
                }

                let value = option_value.unwrap();

                let semicolon_token = self.current_token();

                if semicolon_token.kind != TokenType::Semicolon {
                    println!("Expected Semicolon after declaring var");
                }

                return Ok(Statement{statement_type : Statements::VariableDeclaration(VariableDeclaration{name, value, variable_type}), end_pos: semicolon_token.end_pos, ..statement_default});
            },
            _ => {
                return Err(String::from("Syntax Error!!!"));
            }
        };
    }

    pub fn current_token(&mut self) -> Token {
        let tkn = self.input.get(self.position).unwrap().clone(); 
        
        self.position += 1;
        
        return tkn;
    }

    /*
    pub fn parse_all(
        &mut self,
        stack: &mut Vec<StackFrame>,
        loop_number: u32,
        print_strings: &mut Vec<String>,
        loops: &mut Vec<LoopStruct>,
        compare_number: &mut u32,
        functions: &mut HashMap<String, FunctionStruct>,
        labels: &mut Vec<String>,
        current_offset: &mut u64,
        ) -> Result<String, String> {
        let mut res : String = String::new();

        for item in self.input.iter() {
            match self.parse(item, stack, loop_number, print_strings, loops, compare_number, functions, labels, current_offset) {
                Ok(parsed) => {
                    res.push_str(&parsed as &str);
                },
                Err(err) => {
                    return Err(err);
                }
            };
        }

        return Ok(res);
    }

    pub fn parse(
        &mut self,
        token : &Token,
        stack: &mut Vec<StackFrame>,
        mut loop_number: u32,
        print_strings: &mut Vec<String>,
        loops: &mut Vec<LoopStruct>,
        compare_number: &mut u32,
        functions: &mut HashMap<String, FunctionStruct>,
        labels: &mut Vec<String>,
        current_offset: &mut u64,
        ) -> Result<String, String> {
        match token {
            Token::Asm(asm) => {
                return Ok(asm.to_string());
            },
            Token::Import(file_location) => {
                let mut file = File::open(file_location).expect("error opening file");

                let mut file_content = String::new();

                file.read_to_string(&mut file_content).expect("Error reading as string");
                
                let parsed_file_code = parse_code(&file_content as &str, stack, loop_number, print_strings, loops, compare_number, functions, labels, current_offset);

                match parsed_file_code {
                    Ok(code) => {
                        return Ok(code);     
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            Token::DataBoolean(databoolean) => {
                let ok_offset = get_offset(stack.clone());
                let num : u8 = match databoolean.value {
                    false => 0,
                    true => 1
                };
                let stack_item : StackItem = StackItem{offset: ok_offset, size: 16, variable: VariableType::Bool(databoolean.clone())};
             
                *current_offset += 16;

                let last = stack.last_mut().expect("error getting last mut from stack");
                last.stack_items.insert(databoolean.name.clone(), stack_item);
                
                return Ok(format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16

"#, num));
            }
            Token::CallFunction(func) => {
                let mut res = String::new();
                let mut offset_added : u16 = 0;
                for arg in func.args.clone() {
                    match arg {
                        FunctionArg::Value(val) => {
                            match val {
                                ValueType::Number(num) => {
                                    res.push_str(&format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16
"#, num));
                                    offset_added += 16;
                                },
                                ValueType::Boolean(bool) => {
                                    let num : u8 = match bool {
                                        false => 0,
                                        true => 1,
                                    };
                                    res.push_str(&format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16
"#, num));
                                    offset_added += 16;
                                },
                                _ => {}
                            }
                        },
                        FunctionArg::Variable(var) => {
                            match var.variable {
                                VariableType::Number(num) => {
                                    let var_offset = stack.last().unwrap().stack_items.get(&num.name).unwrap();
                                    println!("var offset {:?}", var_offset.offset.clone());
                                    res.push_str(&format!(r#"
    ldr X1, [sp, #{}]
    str X1, [sp]
    sub sp, sp, #16
"#, (offset_added as u64 + *current_offset) - var_offset.offset as u64));
                                    offset_added += 16;
                                },
                                VariableType::Bool(bool) => {
                                    let var_offset = stack.last().unwrap().stack_items.get(&bool.name).unwrap();
                                    res.push_str(&format!(r#"
    ldr X1, [sp, #{}]
    str X1, [sp]
    sub sp, sp, #16
"#, (offset_added as u64 + *current_offset) - var_offset.offset as u64));
                                    offset_added += 16;
                                },
                                _ => {}
                            }
                        }
                    }
                }
                res.push_str(&format!(
                    r#"    bl f_{}

"#,
                    func.function.name
                ));
                return Ok(res);
            }
            Token::Function(func) => {
                functions.insert(
                    func.name.clone(),
                    FunctionStruct {
                        content: func.content.clone(),
                        name: func.name.clone(),
                        args: func.args.clone(),
                    },
                );
            }
            Token::Terminate() => {
                return Ok(
                    r#"    mov X0, #0
    mov X16, #1
    svc #0x80

"#.to_string());
            }
            Token::Compare(compare_args) => {
                let mut res : String = String::new();

                let current_number = compare_number.clone();
                *compare_number += 1;

                let mut index = 0;
                for arg in compare_args.compare_types.clone() {
                    println!("compare arg: {:?}", arg.clone());
                    match arg {
                        CompareType::Number(num) => {
                            res.push_str(&format!(
                                r#"    mov W{}, #{}

"#,
                                1 + index,
                                num
                            ));
                        }
                        CompareType::Bool(bool) => {
                            let num = match bool {
                                true => 1,
                                false => 0
                            };
                            res.push_str(&format!(
r#"    mov W{}, #{}

"#, 1 + index, num));
                        }
                        CompareType::VariableBool(variable) => {
                            res.push_str(&format!(
                                r#"
    ldr W{}, [sp, #{}]

"#, 1 + index, (*current_offset as u32) - variable.offset));
                        }
                        CompareType::VariableNumber(variable) => {
                            res.push_str(&format!(
                                r#"
    ldr W{}, [sp, #{}]

"#, 1 + index, (*current_offset as u32) - variable.offset));
                        }
                        CompareType::None() => {
                            return Err(String::from("Compare type was not given"));
                        }
                    }
                    index += 1;
                }

                res.push_str(&format!(
                    r#"    cmp W1, W2

"#
                ));
                for symbol in compare_args.symbols.clone() {
                    let shortcut: String;
                    let compare_type: String;

                    match &symbol.symbol as &str {
                        "==" => {
                            shortcut = String::from("eq");
                            compare_type = String::from("equal");
                        }
                        "!=" => {
                            shortcut = String::from("ne");
                            compare_type = String::from("not_equal");
                        }
                        ">=" => {
                            shortcut = String::from("ge");
                            compare_type = String::from("greater_equal");
                        }
                        "<=" => {
                            shortcut = String::from("le");
                            compare_type = String::from("less_equal");
                        }
                        ">" => {
                            shortcut = String::from("gt");
                            compare_type = String::from("greater_than");
                        }
                        "<" => {
                            shortcut = String::from("lt");
                            compare_type = String::from("less_than");
                        }
                        _ => return Err(String::from("Invalid compare syntax.")),
                    }

                    res.push_str(&format!(
                        r#"    b.{} {}_{}

"#,
                        shortcut, compare_type, current_number
                    ));
                }

                res.push_str(&format!(
                    r#"    b continue_{}
"#,
                    current_number
                ));

                for symbol in compare_args.symbols.clone() {
                    let symbol_type: String = match &symbol.symbol as &str {
                        "==" => String::from("equal"),
                        "!=" => String::from("not_equal"),
                        ">=" => String::from("greater_equal"),
                        "<=" => String::from("less_equal"),
                        ">" => String::from("greater_than"),
                        "<" => String::from("less_than"),
                        _ => {
                            return Err(String::from("unknown compare symbol"));
                        }
                    };
                    let parsed_compare_text = parse_code(
                        &symbol.function_content as &str,
                        stack,
                        loop_number,
                        print_strings,
                        loops,
                        compare_number,
                        functions,
                        labels,
                        current_offset,
                    );
                    match parsed_compare_text {
                        Ok(content) => {
                            res.push_str(&format!(
                                r#"{}_{}:
{}

    b continue_{}
"#,
                                symbol_type, current_number, content, current_number
                            ));
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    };
                }

                res.push_str(&format!(
                    r#"

continue_{}:

"#,
                    current_number
                ));

                return Ok(res);
            }
            Token::Number(number) => {
                let ok_offset = get_offset(stack.clone());
                let stack_item : StackItem = StackItem{offset: ok_offset, size: 16, variable: VariableType::Number(number.clone())};
             
                *current_offset += 16;

                let res = format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16

"#, number.value);
                let last = stack.last_mut().expect("error getting last mut from stack");
                last.stack_items.insert(number.name.clone(), stack_item);
                return Ok(res);
            }
            Token::Loop(loop_token) => {
                let num = loop_number;
                loop_number += 1;
                loops.push(LoopStruct {
                    limit: loop_token.number as u32,
                });
                let got_offset = get_offset(stack.clone());
                stack.last_mut().unwrap().stack_items.insert(format!("loop_{}_return", num), StackItem{variable: VariableType::Return(), offset: got_offset, size: 16});
                stack.last_mut().unwrap().stack_items.insert(format!("loop_{}_index", num), StackItem{variable: VariableType::Number(DataNumber{name: format!("loop_{}_index", num), value: 0}), offset: got_offset+16, size: 16});
                *current_offset += 32;
                let stack_last_before = stack.last().unwrap().clone();
                let offset_before = current_offset.clone();
                let compiled_content = parse_code(
                    &loop_token.content as &str,
                    stack,
                    loop_number,
                    print_strings,
                    loops,
                    compare_number,
                    functions,
                    labels,
                    current_offset
                );
                match compiled_content {
                    Ok(content) => {
                        labels.push(format!(r#"
l_{}_start:
    str X30, [sp]
    sub sp, sp, #16

    mov W1, #0
    str W1, [sp]
    sub sp, sp, #16

    b l_{}

l_{}:

{}

    add sp, sp, #{}

    ldr W11, [sp, #16]
    add W11, W11, #1
    str W11, [sp, #16]

    mov W12, #{}
    cmp W12, W11
    b.ne l_{}
 
    ldr X30, [sp, #32]

    add sp, sp, #32

    ret

"#, num, num, num, content, *current_offset - offset_before, loop_token.number, num));
                        stack.last_mut().unwrap().stack_items.remove(&format!("loop_{}_return", num));
                        stack.last_mut().unwrap().stack_items.remove(&format!("loop_{}_index", num));
                        *current_offset -= 32;
                        for item in stack.last().unwrap().stack_items.clone().iter() {
                            let item_from_stack = stack_last_before.stack_items.get(item.0);
                            match item_from_stack {
                                Some(_) => {},
                                None => {
                                    stack.last_mut().unwrap().stack_items.remove(item.0);
                                    *current_offset -= item.1.size as u64;
                                }
                            }
                        }
                        return Ok(format!(
                            r#"    bl l_{}_start

"#, num))
                    }
                    Err(err) => return Err(err),
                }
            }
            Token::WaitNumber(number) => {
                let seconds = number.floor();
                //let nanoseconds = (number - seconds) * 1000000000.0;
                // For future devs: please add a feature to wait .. of nanoseconds.

                // Wait .. seconds and then continue to execute code.
                return Ok(format!(
                    r#"    mov x8, {}
    stp x8, xzr, [sp, -0x10]!
    mov x0, sp
    mov x1, 0
    bl _nanosleep
    add sp, sp, 0x10

"#,
                    seconds
                ));
            }
            Token::PrintlnString(print_string) => {
                // Call print instruction with the print string and add \n to it.
                print_strings.push(print_string.value.clone());
                
                return Ok(format!(
                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#,
                    format!("print_string_{}", print_strings.len() - 1),
                    format!("print_string_{}", print_strings.len() - 1),
                    print_string.length + 1
                ));
            }
            Token::PrintString(print_string) => {
                // Same as printlnString just don't add \n to print_string.
                
                print_strings.push(print_string.value.clone());

                return Ok(format!(
                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#,
                    format!("print_string_{}", print_strings.len() - 1),
                    format!("print_string_{}", print_strings.len() - 1),
                    print_string.length
                ));
            }
            /*
            Token::PrintVariable(name) => {
                let variable = stack.get(&name);

                // Check if variable exists.
                match variable {
                    Some(value) => {
                        // Check if it is a string.
                        match value {
                            // print the variable.
                            VariableType::String(_) => {
                                parsed_text.push_str(&format!(
                                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    adrp X3, {}@PAGE
    add X3, X3, {}@PAGEOFF
    sub X2, X3, X1
    mov X16, #4
    svc #0x80

"#,
                                    name,
                                    name,
                                    format!("{}_end", name),
                                    format!("{}_end", name)
                                ));
                            }
                            _ => {
                                return Err(String::from("variable is not a string!!"));
                            }
                        }
                    }
                    // If variable is not found return.
                    None => {
                        return Err(String::from("no variable found"));
                    }
                };
            }
            Token::PrintlnVariable(name) => {
                let variable = variables.get(&name);

                // Check if variable exists.
                match variable {
                    Some(value) => {
                        // Check if it is a string.
                        match value {
                            // Same as print variable just print new_line after it.
                            VariableType::String(_) => {
                                parsed_text.push_str(&format!(
                                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    adrp X3, {}@PAGE
    add X3, X3, {}@PAGEOFF
    sub X2, X3, X1
    mov X16, #4
    svc #0x80

    mov X0, #1
    adrp X1, new_line@PAGE
    add X1, X1, new_line@PAGEOFF
    mov X2, 1
    mov X16, #4
    svc #0x80

"#,
                                    name,
                                    name,
                                    format!("{}_end", name),
                                    format!("{}_end", name)
                                ));
                            }
                            // Return if not string.
                            _ => {
                                return Err(String::from("variable is not a string!!"));
                            }
                        }
                    }
                    None => return Err(String::from("no variable found")),
                };
            }
            Token::String(string) => {
                // Insert string variable to variables hashmap.
                variables.insert(string.name.clone(), VariableType::String(string));
            }
            */
            Token::Error(err) => {
                // Thow an error and return.
                return Err(err.to_string());
            }
            Token::EOF => {
                // Break if it is end of file.
                println!("Completed!!!");
                return Ok(String::new());
            }
            Token::Comment => {
                // Do nothing if it is comment
            }
            _ => {}
        }

        
        Ok(String::new())
    }
    */
}

pub fn parse_code(
    input: &str,
    ) -> Result<String, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize_all();
    
    let mut parser = Parser::new(&tokens);
    let parsed = parser.parse_all();
    
    let mut analyzer = SemanticAnaytis::new(&parsed, input);
    analyzer.analyze_all();

    let mut code_generator = CodeGenerator::new(&parsed);
    let generated_output = code_generator.generate_all();
    println!("{:?}", generated_output);

    let res : String = generated_output.join("");

    /*
    loop {
        let token = tokenizer.next_token(stack.clone(), functions.clone());
        match token {
            Token::EOF => break,
            Token::Error(err) => return Err(err),
            _ => {
                let mut token_vec : Vec<Token> = Vec::new();
                token_vec.push(token.clone());
                let mut parser = Parser::new(&token_vec);
                let result = parser.parse(&token, stack, loop_number, print_strings, loops, compare_number, functions, labels, current_offset).unwrap();
                res.push_str(&result);
            }
        }
    }
    */
    return Ok(res);
}

pub fn get_offset(stack : Vec<StackFrame>) -> u32 {
    let last_stack = stack.last().expect("error getting stack");

    let mut biggest_offset = 0;

    for item in last_stack.stack_items.values() {
        if item.offset >= biggest_offset {
            biggest_offset = item.offset + item.size;
        }
    }

    if biggest_offset == 0 {
        if stack.len() >= 2 {
            for frame in stack.iter() {
                for item in frame.stack_items.values() {
                    if item.offset > biggest_offset {
                        biggest_offset = item.offset + item.size;
                    }
                }
            }

            return biggest_offset;
        } else {
            return 0;
        }
    } else {
        return biggest_offset;
    }
}
