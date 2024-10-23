use super::{Token, Identifiers, TokenType, DataString, PrintString, FunctionArg, Keywords, BuildInFunctions, BuildInCommand, ValueType, ArgType, DataBoolean, Operators, StackFrame, FunctionStruct, DataNumber, Punctuations, VariableType, LoopToken, Compare, CompareType, CompareSymbol, CallFunction};
use std::num::ParseFloatError;
use std::collections::HashMap;

// Please don't mind my horrible error messages.
// If someone has extra time to waste please rework them.

// Tokenzer struct
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    col: u32,
    line: u32,
}

pub enum GetTokenReturn {
    Instruction(i8),
    Variable(VariableType),
    Function(FunctionStruct)
}

impl<'a> Tokenizer<'a> {
    // Initialize the tokenizer.
    pub fn new(input: &'a str) -> Self {
        Self {input, position: 0, col: 1, line: 1}
    }

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        let mut res : Vec<Token> = Vec::new();

        loop {
            let token = self.next_token();
            
            match token {
                Some(tkn) => {
                    res.push(tkn.clone());
                    if matches!(&tkn.kind, TokenType::EOF) {
                        return res;
                    }
                },
                None => {}
            };
        }
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.input.len() <= self.position {
            return Some(Token{kind: TokenType::EOF, start_col: self.col, end_col: self.col, line: self.line, start_pos: self.position, end_pos: self.position});
        }

        let mut res = String::new();

        let start_col = self.col;
        let start_pos = self.position;

        match self.current_char() {
            '\n' | ';' | '(' | ')' | ',' => {
                self.col += 1;
                res = String::from(self.current_char());
                self.position += 1;
            },
            _ => {
                while self.position < self.input.len() && self.current_char().is_whitespace() == false && matches!(self.current_char(), ';' | '(' | ')' | ',') == false {
                    res.push(self.current_char());
                    self.col += 1;
                    self.position += 1;
                };
            }
        }

        let token_default = Token{kind: TokenType::EOF, start_col, end_col: self.col, line: self.line, start_pos, end_pos: self.position};

        match &res as &str {
            "\n" => {
                self.line += 1;
                self.col = 1;
            },
            "term" => {
                return Some(Token{kind: TokenType::BuildInCommand(BuildInCommand::Terminate), ..token_default})
            },
            ";" => {
                return Some(Token{kind: TokenType::Semicolon, ..token_default});
            },
            "=" => {
                return Some(Token{kind: TokenType::Operator(Operators::Assignment), ..token_default});
            },
            "(" => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::OpenParenthesis), ..token_default})
            },
            ")" => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::ClosedParenthesis), ..token_default})
            },
            "," => {
                return Some(Token{kind: TokenType::Punctuation(Punctuations::Comma), ..token_default})
            },
            "string" => {
                return Some(Token{kind: TokenType::Keyword(Keywords::StringType), ..token_default})
            },
            "number" => {
                return Some(Token{kind: TokenType::Keyword(Keywords::NumberType), ..token_default})
            },
            "compare" => {
                return Some(Token{kind: TokenType::BuildInFunctions(BuildInFunctions::Compare), ..token_default})
            },
            "println" => {
                return Some(Token{kind: TokenType::BuildInFunctions(BuildInFunctions::Println), ..token_default})
            },
            "loop" => {
                return Some(Token{kind: TokenType::BuildInFunctions(BuildInFunctions::Loop), ..token_default})
            },
            _ => {
                match res.parse::<i32>() {
                    Ok(num) => {
                        return Some(Token{kind: TokenType::Identifiers(Identifiers::NumberLiteral(num)), ..token_default});
                    },
                    Err(_) => {
                        return Some(Token{kind: TokenType::Identifiers(Identifiers::VariableName(res)), ..token_default});

                    }
                }
            }
        } 

        return None;
    }

    /*
    pub fn tokenize_all(&mut self, stack : Vec<StackFrame>, functions: HashMap<String, FunctionStruct>) -> Vec<Token> {
        self.position = 0;

        let mut res : Vec<Token> = Vec::new();

        while self.position < self.input.len() {
            let token = self.next_token(stack.clone(), functions.clone());
            if token == Token::EOF {
                println!("res: {:?}", res);
                return res;
            }
            res.push(token);
        }

        return res;
    }

    // Get next token.
    pub fn next_token(&mut self, stack : Vec<StackFrame>, functions: HashMap<String, FunctionStruct>) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        } else {
            let token_return = self.get_token(&stack, &functions);

            // Percentage compiled.
            let percentage = (self.position as f64 / self.input.len() as f64) * 1000.0;
            let rounded_percentage = percentage.round() / 10.0;

            match token_return {
                GetTokenReturn::Instruction(token) => { 
                    match token {
                        -1 => {return Token::EOF;}
                        0 => {
                            self.skip_whitespace();
                            let fn_name = self.get_function_name();
                            if self.current_char() != '(' {return Token::Error(String::from("Expected ( after fn"))};
                            self.position += 1;
                            let fn_args = self.get_function_args_creating();
                            let ok_args = match fn_args {
                                Ok(args) => args,
                                Err(err) => return Token::Error(err),
                            };
                            self.skip_whitespace();
                            if self.current_char() == '{' {
                                self.position += 1;
                                let fn_content = self.get_content_from_braces();
                                return Token::Function(FunctionStruct{name: fn_name, content: fn_content, args: ok_args});
                            } else {
                                return Token::Error(String::from("Expected { after fn"));
                            }
                        },
                        1 => {
                            if self.current_char() == ';' {
                                self.position += 1;
                                return Token::Terminate();
                            } else {
                                return Token::Error(String::from("Expected ; after term"));
                            }
                        },
                        2 => {
                            if self.current_char().is_whitespace() == false {
                                return Token::Error(String::from("Expected Whitespace"));
                            }

                            self.position += 1;
                            let bool_name = self.get_text();
                            self.skip_whitespace();
                            if self.current_char() != '=' {
                                return Token::Error(String::from("Expected = after bool"));
                            }

                            self.position += 1;
                            self.skip_whitespace();

                            let bool_value : String = self.get_boolean_value();

                            self.skip_whitespace();

                            if self.current_char() != ';' {
                                return Token::Error(String::from("Expected ;"));
                            }

                            self.position += 1;

                            let boolean_value : bool = match &bool_value as &str {
                                "false" => false,
                                "true" => true,
                                _ => {return Token::Error(String::from("Invalid bool value"))}
                            };

                            return Token::DataBoolean(DataBoolean{name: bool_name, value: boolean_value});
                        },
                        3 => {
                            // String variable
                            self.skip_whitespace();

                            let string_name = self.get_text();

                            self.skip_whitespace();

                            if self.current_char() == '=' {
                                self.position += 1;
                                self.skip_whitespace();

                                if self.current_char() == '"' {
                                    self.position += 1;

                                    // Get init value from string.
                                    let string_value : String = self.get_string_value();

                                    self.skip_whitespace();

                                    if self.current_char() == ';' {
                                        self.position += 1;
                                        return Token::String(DataString{name: string_name, value: string_value})
                                    } else {
                                        return Token::Error(String::from("Expected ;"));
                                    }
                                } else {
                                    return Token::Error(String::from("Expected String!!"));                              
                                }
                            } else {
                                return Token::Error(format!("Expected = after string {}", string_name))
                            }
                        }
                        4 => {
                            // Wait set amount of time
                            if self.current_char() == '(' {
                                self.position += 1;
                                let number = self.get_number_from_wait().expect("Did you use a number at wait?");

                                if self.current_char() == ';' {
                                    self.position += 1;
                                    return Token::WaitNumber(number);
                                } else {
                                    return Token::Error(String::from("Expected ; after wait()"));
                                }
                            } else {
                                return Token::Error(String::from("Expected ( After Wait"))
                            }
                        },
                        5 => {
                            if self.current_char() == '(' {
                                self.position += 1;
                                if self.current_char() == '"' {
                                    self.position += 1;

                                    let mut print_string = self.get_print_properties();
                                
                                    // Add newline to print string.
                                    print_string.value.push_str("\\n");

                                    if self.current_char() == ')' {
                                        self.position += 1;
                                        if self.current_char() == ';' {
                                            self.position += 1;
                                            return Token::PrintlnString(print_string);
                                        } else {
                                            return Token::Error(String::from("Expected ; To Close Line"));
                                        }
                                    } else {
                                        return Token::Error(String::from("Expected ) on Print Function"))
                                    }
                                } else {
                                    // Get name of string value from ().
                                    let string_var_name = self.get_value_from_parentheses();

                                    if self.current_char() == ';' {
                                        self.position += 1;
                                        return Token::PrintlnVariable(string_var_name);
                                    } else {
                                        return Token::Error(String::from("Expected ; after print statement!!"));
                                    }
                                }
                            } else {
                                return Token::Error(String::from("expected ( after println"));
                            }
                        },
                        6 => {
                            if self.current_char() == '(' {
                                self.position += 1;
                                if self.current_char() == '"' {
                                    self.position += 1;

                                    let print_string = self.get_print_properties();
    
                                    if self.current_char() == ')' {
                                        self.position += 1;
                                        if self.current_char() == ';' {
                                            self.position += 1;
                                            return Token::PrintString(print_string);
                                        } else {
                                            return Token::Error(String::from("Expected ; To Close Line"));
                                        }
                                    } else {
                                        return Token::Error(String::from("Expected ) on Print Function"))                                
                                    }
                                } else {
                                    let string_var_name = self.get_value_from_parentheses();

                                    if self.current_char() == ';' {
                                        self.position += 1;
                                        return Token::PrintVariable(string_var_name);
                                    } else {
                                        return Token::Error(String::from("Expected ; after print statement!!"));
                                    }
                                }
                            } else {
                                return Token::Error(String::from("Expected ( after print"));
                            }
                        },
                        7 => {
                            if self.current_char() == '(' {
                                self.position += 1;
                                // Get both inputs from compare(input1, input2);.
                                let compares : Result<[CompareType; 2], String> = self.get_compare_args(stack);

                                match compares {
                                    Ok(compare) => {
                                        // Check if both inputs are numbers.
                                        if (matches!(compare[0], CompareType::Number(_) | CompareType::VariableNumber(_)) && matches!(compare[1], CompareType::Number(_) | CompareType::VariableNumber(_))) || (matches!(compare[0], CompareType::Bool(_) | CompareType::VariableBool(_)) && matches!(compare[1], CompareType::Bool(_) | CompareType::VariableBool(_))) {
                                            self.skip_whitespace();
                                            if self.current_char() == ';' {
                                                return Token::Error(String::from("Please use compare or remove it"));
                                            } else {
                                                let mut compare_symbols : Vec<CompareSymbol> = Vec::new();

                                                loop {
                                                    self.skip_whitespace();
                                                    if self.current_char() == ';' {self.position += 1; break;} else if self.current_char() == '.' {
                                                        self.position += 1;
                                                        let syntax = &self.input[self.position..self.position + 2];
                                                        match syntax {
                                                            "==" | "!=" | ">=" | "<=" => {
                                                                self.position += 2;
                                                                self.skip_whitespace();
                                                                if self.current_char() == '{' {
                                                                    self.position += 1;
                                                                    let func_content = self.get_content_from_braces();
                                                                    let compare_symbol = CompareSymbol{symbol : syntax.to_string(), function_content: func_content};
                                                                    compare_symbols.push(compare_symbol);
                                                                } else {
                                                                    return Token::Error(String::from("expected { after .XX"))
                                                                }
                                                            },
                                                            _ => {
                                                                let one_char_syntax = &self.input[self.position..self.position + 1];
                                                                match one_char_syntax {
                                                                    ">" | "<" => {
                                                                        self.position += 2;
                                                                        self.skip_whitespace();
                                                                        if self.current_char() == '{' {
                                                                            self.position += 1;
                                                                            let func_content = self.get_content_from_braces();
                                                                            let compare_symbol = CompareSymbol{symbol : one_char_syntax.to_string(), function_content: func_content};
                                                                            compare_symbols.push(compare_symbol);
                                                                        } else {
                                                                            return Token::Error(String::from("expected { after .XX"))
                                                                        }
                                                                    }
                                                                    _ => {return Token::Error(String::from("Invalid Comparing Syntax"));}  
                                                                };
                                                            }
                                                        }
                                                    } else {
                                                        return Token::Error(String::from("syntax error"));
                                                    }
                                                }
                                                let compare : Compare = Compare{compare_types : compare, symbols : compare_symbols};

                                                // End
                                                return Token::Compare(compare);
                                            }
                                        } 
                                        else {
                                            return Token::Error(String::from("Please provide valid compare"))
                                        }
                                    },
                                    Err(error) => {
                                        return Token::Error(error);
                                    }
                                }
                            } else {
                                return Token::Error(String::from("Expected ( after compare"));
                            }
                        },
                        8 => {
                            // Creating number variable.
                            self.skip_whitespace();
                            let variable_name = self.get_text();
                            self.skip_whitespace();
                            if self.current_char() == '=' {
                                self.position += 1;
                                self.skip_whitespace();
                                // Get init value for number variable.
                                let number_value = self.get_number_from_number_variable_init();
                                self.skip_whitespace();
                                if self.current_char() == ';' {
                                    self.position += 1;
                                    let data_number : DataNumber = DataNumber{value: number_value, name: variable_name};
                                    return Token::Number(data_number);
                                } else {
                                    return Token::Error(String::from("Expected ; after number"));
                                }
                            } else {
                                return Token::Error(String::from("Expected = after number"));
                            }
                        },
                        9 => {
                            // Loop fixed amount of times.
                            if self.current_char() == '(' {
                                self.position += 1;
                                let loop_number : i32 = self.get_number_from_loop();
    
                                if self.current_char() == ')' {
                                    self.position += 1;
                                    self.skip_whitespace();
                                    if self.current_char() == '{' {
                                        self.position += 1;
                                        let loop_content = self.get_content_from_braces();
                                        return Token::Loop(LoopToken{content: loop_content, number: loop_number});
                                    } else {
                                        return Token::Error(String::from("Expected { after loop"));
                                    }
                                } else {
                                    return Token::Error(String::from("Expected ) after loop(number"))
                                }
                            } else {
                                return Token::Error(String::from("Expected ( after loop"));
                            }
                        },
                        10 => {
                            // Skip text if it is a comment.
                            self.handle_comment();
                            return Token::Comment;
                        },
                        11 => {
                            // Import file
                            self.skip_whitespace();
                            if self.current_char() != '"' {
                                return Token::Error(String::from("Expected \" after import"));
                            }

                            self.position += 1;

                            let content = self.get_string_value();

                            println!("{:?}", content);

                            if self.current_char() != ';' {
                                return Token::Error(String::from("Expected ; after import"));
                            }

                            self.position += 1;

                            return Token::Import(content);
                        },
                        12 => {
                            println!("asm!!!!!!");
                            self.skip_whitespace();
                            if self.current_char() != '<' {
                                return Token::Error(String::from("expected < after asm"));
                            } 
                            
                            self.position += 1;

                            let content = self.get_content_from_asm();
                            return Token::Asm(content);
                        },
                        _ => {
                                return Token::Error(format!("Unknown Character {}", self.current_char()));
                        }
                    }
                },
                GetTokenReturn::Variable(var) => {return Token::Error(String::from("got token variable"))},
                GetTokenReturn::Function(func) => {
                    if self.current_char() != '(' {
                        return Token::Error(String::from("Expected ( after calling function"));
                    }
                    let args = match self.get_function_args(stack) {
                        Ok(args) => args,
                        Err(err) => return Token::Error(err),
                    };
                    if args.len() != func.args.len() {
                        return Token::Error(String::from("Invalid number of args passed"));
                    }
                    if self.current_char() != ';' {
                        return Token::Error(String::from("Expected ; after calling function"));
                    }
 
                    self.position += 1;

                    return Token::CallFunction(CallFunction{function: func, args});
                }
            }
        }
    }
    */

    pub fn get_content_from_asm(&mut self) -> String {
        let mut res : String = String::new();

        while self.position < self.input.len() {
            if self.current_char() == '>' {
                self.position += 1;
                break;
            } else {
                res.push(self.current_char());
                self.position += 1;
            }
        }

        return res;
    }

    pub fn get_function_args_creating(&mut self) -> Result<Vec<ArgType>, String> {
        let mut args : Vec<ArgType> = Vec::new();

        loop {
            if self.current_char() == ')' {
                self.position += 1;
                break;
            } else if self.current_char() == ',' {self.position += 1;};

            self.skip_whitespace();

            let mut arg_type : String = String::new();
            let mut arg_name : String = String::new();

            loop {
                if self.current_char().is_whitespace() {
                    self.skip_whitespace();
                    break;
                }
                arg_type.push(self.current_char());
                self.position += 1;
            }

            loop {
                if self.current_char() == ',' || self.current_char() == ' ' || self.current_char() == ')' {
                    self.skip_whitespace();
                    break;
                } else {
                    arg_name.push(self.current_char());
                    self.position += 1;
                }
            }

            let arg = match &arg_type as &str {
                "bool" => ArgType::Bool(arg_name),
                "number" => ArgType::Number(arg_name),
                "string" => ArgType::String(arg_name),
                _ => {return Err(String::from("Invalid Arg Type!!!"))}
            };

            args.push(arg);
        }

        return Ok(args);
    }

    pub fn get_token(&mut self, stack : &Vec<StackFrame>, functions: &HashMap<String, FunctionStruct>) -> GetTokenReturn {
        let tokens : HashMap<&str, i8> = HashMap::from([("fn", 0), ("term", 1), ("bool", 2),/* ("string", 3),*/ ("wait", 4), ("println", 5), ("print", 6), ("compare", 7), ("number", 8), ("loop", 9), ("\\\\", 10), ("import", 11), ("asm", 12)]);

        let mut res : String = String::new();

        while self.position < self.input.len() {
            match self.current_char() {
                '(' | ')' | ';' | ' ' | '<' | '>' | '\n' => {
                   let token = tokens.get(&res as &str);
                    match token {
                        Some(token) => {
                            return GetTokenReturn::Instruction(*token);
                        },
                        None => {}
                    };
                    let variable = stack.last().unwrap().stack_items.get(&res);
                    match variable {
                        Some(var) => {
                            return GetTokenReturn::Variable(var.variable.clone());
                        },
                        None => {}
                    }
                    let function = functions.get(&res as &str);
                    match function {
                        Some(func) => {
                            return GetTokenReturn::Function(func.clone());
                        },
                        None => {}
                    }
                    break;
                },
                _ => {
                    res.push(self.current_char());
                    self.position += 1;
                }
            }
            
        };

        return GetTokenReturn::Instruction(-1);
    }

    pub fn get_boolean_value(&mut self) -> String {
        let mut res : String = String::new();
        
        while self.position < self.input.len() {
            if self.current_char().is_whitespace() || self.current_char() == ';' {
                break;
            } else {
                res.push(self.current_char());
                self.position += 1;
            }
        }

        return res;
    }

    pub fn get_content_from_braces(&mut self) -> String {
        let mut result : String = String::new();

        let mut braces = 0;

        while self.position < self.input.len() {
            if self.current_char() == '}' {
                if braces == 0 {
                    self.position += 1;
                    break;
                } else {
                    result.push(self.current_char());
                    self.position += 1;
                    braces -= 1;
                }
            } else if self.current_char() == '{' {
                result.push(self.current_char());
                self.position += 1;
                braces += 1;
            } else {
                result.push(self.current_char());
                self.position += 1;
            }
        };

        return result;
    }

    // Get arguments from compare(arg1, arg2); and return error or result.
    pub fn get_compare_args(&mut self, stack : Vec<StackFrame>) -> Result<[CompareType; 2], String> {
        self.skip_whitespace();
        let mut arg1 = String::new();
        let mut arg2 = String::new();

        while self.position < self.input.len() && self.current_char().is_whitespace() == false && self.current_char() != ',' {
                arg1.push(self.current_char());
                self.position += 1;
        };

        self.skip_whitespace();

        if self.current_char() != ',' {
            return Err(String::from("Expected , after compare arg1"))
        }

        self.position += 1;
        self.skip_whitespace();
        
        while self.position < self.input.len() && self.current_char().is_whitespace() == false && self.current_char() != ')' {
                arg2.push(self.current_char());
                self.position += 1;
        };

        self.skip_whitespace();

        if self.current_char() != ')' {
            return Err(String::from("Expected ) after compare!!"))
        }
        self.position += 1;

        let mut result : [CompareType; 2] = [CompareType::None(), CompareType::None()];

        let arg1var = stack.last().unwrap().stack_items.get(&arg1);
        let arg2var = stack.last().unwrap().stack_items.get(&arg2);

        match arg1var {
            Some(variable) => {
                match &variable.variable {
                    VariableType::Number(_) => {
                        result[0] = CompareType::VariableNumber(variable.clone());
                    },
                    VariableType::Bool(_) => {
                        result[0] = CompareType::VariableBool(variable.clone());
                    },
                    _ => {
                        println!("Invalid Variable Used In Compare");
                    }
                }
            },
            None => {
                let isnumber = self.validate_number_from_string(arg1.clone());
                if isnumber {
                    let converted_number : i64 = arg1.clone().parse::<i64>().expect("error parsing to number");
                    result[0] = CompareType::Number(converted_number)
                } else {
                    match &arg1 as &str {
                        "true" => result[0] = CompareType::Bool(true),
                        "false" => result[0] = CompareType::Bool(false),
                        _ => {return Err(String::from("Invalid Value In Compare"))}
                    }
                }
            }
        }

        match arg2var {
            Some(variable) => {
                match &variable.variable {
                    VariableType::Number(_) => {
                        result[1] = CompareType::VariableNumber(variable.clone());
                    },
                    VariableType::Bool(_) => {
                        result[1] = CompareType::VariableBool(variable.clone());
                    },
                    _ => {
                        println!("Invalid Variable Used In Compare");
                    }
                }
            },
            None => {
                let isnumber = self.validate_number_from_string(arg2.clone());
                if isnumber {
                    let converted_number : i64 = arg2.clone().parse::<i64>().expect("error parsing to number");
                    result[1] = CompareType::Number(converted_number)
                } else {
                    match &arg2 as &str {
                        "true" => result[1] = CompareType::Bool(true),
                        "false" => result[1] = CompareType::Bool(false),
                        _ => {return Err(String::from("Invalid Value In Compare"))}
                    }
                }
            }
        }

        return Ok(result);
    }

    // Check if string is full of numbers.
    pub fn validate_number_from_string(&mut self, string : String) -> bool {
        for char in string.chars() {
            if char.is_numeric() == false {
                return false;
            }
        }

        return true;
    }

    // Get number from loop loop(100) -> 100.
    pub fn get_number_from_loop(&mut self) -> i32 {
        let start = self.position;
        let mut end = self.position;

        while self.position < self.input.len() {
            if self.current_char() == ')' {
                end = self.position;
                break;
            } else {
                self.position += 1;
            }
        }

        let result : i32 = self.input[start..end].parse::<i32>().expect("Did you pass number to loop()?");

        return result
    }

    // Get number from wait wait(5.2) -> 5.2 or error if it is not a number.
    pub fn get_number_from_wait(&mut self) -> Result<f64, ParseFloatError> {
        let start = self.position;
        let mut end = self.position;

        while self.position < self.input.len() {
            if self.current_char() == ')' {
                end = self.position;
                self.position += 1;
                break;
            } else {
                self.position += 1;
            }
        }

        let result : Result<f64, ParseFloatError> = self.input[start..end].parse::<f64>();

        return result;
    }

    // Get init value of number variable - number num = 10; -> 10 
    pub fn get_number_from_number_variable_init(&mut self) -> i64 {
        let mut result : String = String::new();

        while self.position < self.input.len() {
            if self.current_char() == ';' || self.current_char().is_whitespace() {
                break;
            } else {
                result.push(self.current_char());
                self.position += 1;
            }
        };

        let num_result : i64 = result.parse::<i64>().expect("error converting to number");

        return num_result;
    }

    // Handle function arguments.
    pub fn get_function_args(&mut self, stack : Vec<StackFrame>) -> Result<Vec<FunctionArg>, String> {
        let mut args : Vec<FunctionArg> = Vec::new();

        let last_frame : StackFrame = stack.last().unwrap().clone();

        if self.current_char() != '(' {
            println!("Expected Arguments After Function Name!!");
        }

        self.position += 1;

        while self.position < self.input.len() {
            let mut res : String = String::new();

            self.skip_whitespace();

            loop {
                if self.current_char() == ',' || self.current_char() == ')' || self.current_char() == ' ' {
                    if let Some(var) = last_frame.stack_items.get(&res) {
                        args.push(FunctionArg::Variable(var.clone()));
                        break;
                    } else {
                        match &res as &str {
                            "true" => args.push(FunctionArg::Value(ValueType::Boolean(true))),
                            "false" => args.push(FunctionArg::Value(ValueType::Boolean(false))),
                            _ => {
                                let number_check = self.validate_number_from_string(res.clone());
                                if number_check {
                                    let num_from_str : i64 = res.parse::<i64>().unwrap();
                                    args.push(FunctionArg::Value(ValueType::Number(num_from_str)));
                                } else {
                                    return Err(String::from("Invalid Arg"));
                                }
                            },
                        };
                        break;
                    }
                } else if self.current_char() == '"' {
                    let mut str : String = String::new();

                    self.position += 1;

                    while self.position < self.input.len() {
                        if self.current_char() == '"' {
                            self.position += 1;
                            break;
                        }

                        str.push(self.current_char());
                        self.position += 1;
                    }

                    args.push(FunctionArg::Value(ValueType::String(str)))
                } else {
                    res.push(self.current_char());
                    self.position += 1;
                }
            }
            
            if self.current_char() == ')' {
                self.position += 1;
                break;
            } else {
                self.position += 1;
            }
        }

        println!("function args: {:?}", args);

        return Ok(args);
    }

    // Get name from function.
    pub fn get_function_name(&mut self) -> String {
        let mut result : String = String::new();

        while self.position < self.input.len() {
            if self.current_char() == '(' {
                break;
            } else {
                result.push(self.current_char());
                self.position += 1;
            }
        }

        return result;
    }

    // Handle comments and closing of comments.
    pub fn handle_comment(&mut self) {
        while self.position < self.input.len() {
            if self.current_char() == '\\' {
                self.position += 1;
                if self.current_char() == '\\' {
                    self.position += 1;
                    return;
                } else {
                    self.position += 1;
                }
            } else {
                self.position += 1;
            }
        }
    }

    // Get value from parentheses - (value) -> value
    pub fn get_value_from_parentheses(&mut self) -> String {
        let mut result : String = String::new();

        while self.position < self.input.len() && self.current_char() != ')' {
            result.push(self.current_char());
            self.position += 1;
        }

        self.position += 1;

        return result;
    }

    // Get value from string - "value" -> value
    pub fn get_string_value(&mut self) -> String {
        let mut result : String = String::new();

        while self.position < self.input.len() && self.current_char() != '"' {
            result.push(self.current_char());
            self.position += 1;
        }

        self.position += 1;

        return result;
    }

    // Handle things like \n etc.
    pub fn get_print_properties(&mut self) -> PrintString {
        let mut result : String = String::new();
        let mut size : u32 = 0;

        while self.position < self.input.len() {
            if self.current_char() == '"' {
                self.position += 1;
         break
            }
            else if self.current_char() == '\\' {
                self.position += 1;
                match self.current_char() {
                    '\\' => {
                        result.push(self.current_char());
                        result.push(self.current_char());
                        self.position += 2;
                        size += 1;
                    },
                    'n' => {
                        result.push('\\');
                        result.push('n');
                        size += 1;
                        self.position += 1;
                    }
                    _ => {}
                }
            } else {
                result.push(self.current_char());
                self.position += 1;
                size += 1;
            }
        }
 
        PrintString{value: result, length: size}
    }

    // Get value until it is not whitespace - val ue -> val
    pub fn get_text(&mut self) -> String {
        let mut result : String = String::new();
    
        while self.position < self.input.len() && self.current_char().is_whitespace() == false {
            result.push(self.current_char());
            self.position += 1;
        }

        return result;
    }

    // Skips whitespace.
    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.col += 1;
            self.position += 1;
        }
    }

    // Get current char of input.
    pub fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    // Get function call name - call function(); - function
    pub fn get_function_call_name(&mut self) -> String {
        let mut res : String = String::new();

        while self.position < self.input.len() {
            if self.current_char() == '(' {
                break;
            } else {
                res.push(self.current_char());
                self.position += 1;
            }
        }

        return res;
    }
 }
