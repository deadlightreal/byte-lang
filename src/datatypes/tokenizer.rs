use super::{Token, DataString, PrintString, DataBoolean, FunctionStruct, DataNumber, VariableType, LoopToken, Compare, CompareType, CompareSymbol};
use std::num::ParseFloatError;
use std::collections::HashMap;

// Please don't mind my horrible error messages.
// If someone has extra time to waste please rework them.

// Tokenzer struct
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

pub enum GetTokenReturn {
    Instruction(i8),
    Variable(VariableType),
    Function(FunctionStruct)
}

impl<'a> Tokenizer<'a> {
    // Initialize the tokenizer.
    pub fn new(input: &'a str) -> Self {
        Self {input, position: 0}
    }

    // Get next token.
    pub fn next_token(&mut self, variables : HashMap<String, VariableType>, functions: HashMap<String, FunctionStruct>) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        } else {
            let token_return = self.get_token(&variables, &functions);

            // Percentage compiled.
            let percentage = (self.position as f64 / self.input.len() as f64) * 1000.0;
            let rounded_percentage = percentage.round() / 10.0;

            println!("{}%", rounded_percentage);

            match token_return {
                GetTokenReturn::Instruction(token) => { 
                    match token {
                        -1 => {return Token::EOF;}
                        0 => {
                            self.skip_whitespace();
                            let fn_name = self.get_function_name();
                            self.handle_function_args();
                            self.skip_whitespace();
                            if self.current_char() == '{' {
                                self.position += 1;
                                let fn_content = self.get_content_from_braces();
                                return Token::Function(FunctionStruct{name: fn_name, content: fn_content});
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
                                let compares : Result<[CompareType; 2], String> = self.get_compare_args(variables);
                                match compares {
                                    Ok(compare) => {
                                        // Check if both inputs are numbers.
                                        if (matches!(compare[0], CompareType::Number(_) | CompareType::VariableNumber(_)) && matches!(compare[1], CompareType::Number(_) | CompareType::VariableNumber(_))) {
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
                    self.position += 1;
                    // Add args later.
                    self.position += 1;
                    if self.current_char() != ';' {
                        return Token::Error(String::from("Expected ; after calling function"));
                    }

                    self.position += 1;

                    return Token::CallFunction(func.name);
                }
            }
        }
    }

    pub fn get_token(&mut self, variables : &HashMap<String, VariableType>, functions: &HashMap<String, FunctionStruct>) -> GetTokenReturn {
        let tokens : HashMap<&str, i8> = HashMap::from([("fn", 0), ("term", 1), ("bool", 2), ("string", 3), ("wait", 4), ("println", 5), ("print", 6), ("compare", 7), ("number", 8), ("loop", 9), ("\\\\", 10), ("import", 11)]);

        let mut res : String = String::new();

        while self.position < self.input.len() {
            match self.current_char() {
                '(' | ')' | ';' | ' ' => {
                    println!("gotten token: {}", res);
                   let token = tokens.get(&res as &str);
                    match token {
                        Some(token) => {
                            return GetTokenReturn::Instruction(*token);
                        },
                        None => {}
                    };
                    let variable = variables.get(&res as &str);
                    match variable {
                        Some(var) => {
                            return GetTokenReturn::Variable(var.clone());
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
    pub fn get_compare_args(&mut self, variables : HashMap<String, VariableType>) -> Result<[CompareType; 2], String> {
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

        let arg1var = variables.get(&arg1);
        let arg2var = variables.get(&arg2);

        match arg1var {
            Some(variable) => {
                match variable {
                    VariableType::Number(number) => {
                        result[0] = CompareType::VariableNumber(number.clone())
                    },
                    _ => {}
                }
            },
            None => {
                let isnumber = self.validate_number_from_string(arg1.clone());
                if isnumber {
                    let converted_number : i64 = arg1.clone().parse::<i64>().expect("error parsing to number");
                    result[0] = CompareType::Number(converted_number)
                } else {
                    return Err(String::from("please provide a valid variable or value"));
                }
            }
        }

        match arg2var {
            Some(variable) => {
                match variable {
                    VariableType::Number(number) => {
                        result[1] = CompareType::VariableNumber(number.clone())
                    },
                    _ => {}
                }
            },
            None => {
                let isnumber = self.validate_number_from_string(arg2.clone());
                if isnumber {
                    let converted_number : i64 = arg2.clone().parse::<i64>().expect("error parsing to number");
                    result[1] = CompareType::Number(converted_number)
                } else {
                    return Err(String::from("please provide a valid variable or value"));
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

    // Handle function arguments - does nothing for now.
    pub fn handle_function_args(&mut self) {
        if self.current_char() == '(' {
            self.position += 1;
            if self.current_char() == ')' {
                self.position += 1;
            } else {

            }
        } else {
            println!("Expected Arguments After Function Name!!")
        }
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
        while self.position < self.input.len() && self.current_char().is_whitespace() {
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
