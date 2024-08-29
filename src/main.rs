use std::io::{BufWriter, Write};
use std::{fs::File, io::Read};
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

mod compile_asm;
mod datatypes;

use compile_asm::compile_asm;
use datatypes::{DataString, Token, CompareSymbol, CompareType, Tokenizer, DataNumber, VariableType};

fn main() {
    let command = std::env::args().nth(1).expect("Please Provide a command");
    match command.as_str() {
        "run" => {
            run_file();
        },
        _ => {}
    }
}

fn run_file() {
    // Getting second arg that should provide location of file that they want to run.
    let file_location = std::env::args().nth(2).expect("Please Provide File Location");

    // Open the file.
    let mut file = File::open(file_location).expect("Error Oppening File");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("Error Reading As String");

    // Get the path that user is in when running the run command!
    let current_dir = std::env::current_dir().expect("Error getting current Path");


    // Create a file that will contain output assembly code.
    let mut new_file_location = PathBuf::from(current_dir.clone());
    new_file_location.push("output.s");
    let created_file = File::create(new_file_location.clone()).expect("Error creating File");

    // Create a writer for assembly code.
    let mut writer = BufWriter::new(created_file);

    // Compile Code.
    //as -o output.o output.s
    //ld -macos_version_min 11.0.0 -o output output.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64

    // Create _start function.
    write!(writer, 
r#".global _start
.align 2
_start:

"#).expect("Error Writing File");

    // Init tokenizer.
    let mut tokenizer = Tokenizer::new(&content);

    // Store strings used to print.
    let mut print_strings : Vec<String> = Vec::new();

    // Variables like numbers and strings.
    let mut variables : HashMap<String, VariableType> = HashMap::new();

    // Store number of loops made.
    let mut loop_number : u32 = 0;

    let mut compare_symbols : Vec<Vec<CompareSymbol>> = Vec::new();
    
    let parsed_text = handle_parsing(tokenizer, &mut variables, loop_number, &mut print_strings, &mut compare_symbols);

    match parsed_text {
        Ok(text) => write!(writer, "{}", text).expect("error writing to a file"),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    let mut index = 0;

    let mut symbols_index : usize = 0;

    while symbols_index < compare_symbols.len() {
        for symbol in compare_symbols[symbols_index].clone() {
            let symbol_tokenizer = Tokenizer::new(&symbol.function_content);
            let symbol_type : String = match &symbol.symbol as &str {
                "==" => String::from("equal"),
                "!=" => String::from("not_equal"),
                _ => {
                    return;
                }
            };
            let parsed = handle_parsing(symbol_tokenizer, &mut variables, loop_number, &mut print_strings, &mut compare_symbols);
            match parsed {
                Ok(text) => {
                    write!(writer,
r#"{}_{}:
{}

    bl continue_{}
"#, symbol_type, symbols_index, text, symbols_index).expect("error writing to file");
                },
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            }
        }

        symbols_index += 1;
    }
    
    write!(writer, ".data\n").expect("error writing to a file");

    // Create a new_line string that contains \n.
    write!(writer, "new_line: .ascii \"\\n\"\n").expect("Error Writing to a file");
    
    // Insert print strings into data section.
    for print_string in print_strings {
        write!(writer, "print_string_{}: .ascii \"{}\"\n", index, print_string).expect("Error writing to a file");
        index += 1;
    }

    // Write variables at the data section.
    for item in variables.values() {
        // Match the variable type.
        match item {
            VariableType::String(string) => {
                write!(writer, "{}: .asciz \"{}\"\n{}_end:\n{}_length: .word {} \n", string.name.clone(), string.value, string.name, string.name, string.value.len()).expect("Error Writing to a file");
            },
            VariableType::Number(number) => {
                write!(writer, "{}: .word {}\n", number.name, number.value).expect("error writing to file");
                println!("number variable {:?}", number);
            }
        }
    }

    // Save the file with new content.
    writer.flush().expect("Err Flushing To File");

    // Compile the assembly file.
    compile_asm(current_dir);
    
    println!("Starting App \n \n \n--------------------------------------------------------------\n \n \n");

    // Run the app.
    let status = Command::new("./output")
        .status()
        .expect("error executing command");

    // Return if failed to run.
    if status.success() == false {
        println!("error running program");
        return;
    }
}

fn handle_parsing(mut tokenizer : Tokenizer, variables : &mut HashMap<String, VariableType>, mut loop_number: u32, print_strings : &mut Vec<String>, compare_symbols :&mut  Vec<Vec<CompareSymbol>>) -> Result<String, String> {
    let mut parsed_text = String::new();

    loop {
        let token : Token = tokenizer.next_token(variables.clone());

        match token {
            Token::Terminate() => {
                parsed_text.push_str(r#"    mov X0, #0
    mov X16, #1
    svc #0x80

"#);
            },
            /*Token::CompareSymbol(symbol) => {
                            */
//r#"    bl continue_{}

//continue_{}:

//"#, 0, 0));
            Token::Compare(compare_args) => {
                let inx = compare_symbols.len();

                compare_symbols.push(Vec::new());
                let mut index = 0;
                for arg in compare_args.compare_types {
                    match arg {
                        CompareType::Number(num) => {
                            parsed_text.push_str(&format!( 
r#"    mov W{}, #{}

"#, 1+index, num));
                        },
                        CompareType::VariableNumber(variable) => {
                            parsed_text.push_str(&format!(
r#"    adrp X3, {}@PAGE
    add X3, X3, {}@PAGEOFF
    ldr W{}, [X3]

"#, variable.name, variable.name, 1 + index));
                        },
                        CompareType::None() => {
                            return Err(String::from("Compare type was not given"));
                        }
                    }
                    index += 1;
                };

                parsed_text.push_str(&format!( 
r#"    cmp W1, W2

"#));
                for symbol in compare_args.symbols {
                    let mut shortcut : String = String::new();
                    let mut compare_type : String = String::new();

                    match &symbol.symbol as &str {
                        "==" => {
                            shortcut = String::from("eq");
                            compare_type = String::from("equal");
                        },
                        "!=" => {
                            shortcut = String::from("ne");
                            compare_type = String::from("not_equal");
                        },
                        _ => {return Err(String::from("Invalid compare syntax."))},
                    }

                    parsed_text.push_str(&format!(
r#"    b.{} {}_{}

"#
, shortcut, compare_type, inx));
                    if let Some(last) = compare_symbols.last_mut() {
                        last.push(symbol);
                    } else {
                        return Err(String::from("Error getting last vector of compare symbols."));
                    }
                };

                parsed_text.push_str(&format!(
r#"    bl continue_{}

continue_{}:

"#, inx, inx));

                println!("compare symbols : {:?}", compare_symbols);
            },
            Token::Number(number) => {
                // Insert into variables hashmap the number variable.
                variables.insert(number.name.clone(), VariableType::Number(number));
            },
            Token::LoopEnd() => {
                // End the loop if index reached the limit number and branch to loop_end.
                parsed_text.push_str(&format!(
r#"    cmp X12, X11
    b.eq l_{}_end

    add X11, X11, #1
    bl l_{}

l_{}_end:

"#, loop_number, loop_number, loop_number));
                loop_number += 1;
            },
            Token::LoopStart(number) => {
                // branch to the loop and initialize limit and index.
                parsed_text.push_str(&format!( 
r#"    mov X12, #{}
    mov X11, #1
    bl l_{}

l_{}:

"#, number, loop_number, loop_number));
            },
            Token::WaitNumber(number) => {
                let seconds = number.floor();
                let nanoseconds = (number - seconds) * 1000000000.0;
                // For future devs: please add a feature to wait .. of nanoseconds.
                
                // Wait .. seconds and then continue to execute code.
                parsed_text.push_str(&format!(
r#"    mov x8, {}
    stp x8, xzr, [sp, -0x10]!
    mov x0, sp
    mov x1, 0
    bl _nanosleep
    add sp, sp, 0x10

"#, seconds));

            },
            Token::PrintlnString(print_string) => {
                // Call print instruction with the print string and add \n to it.
                parsed_text.push_str(&format!(
r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#, format!("print_string_{}", print_strings.len()), format!("print_string_{}", print_strings.len()), print_string.length+1
                ));
                print_strings.push(print_string.value);
            },
            Token::PrintString(print_string) => {
                // Same as printlnString just don't add \n to print_string.
                parsed_text.push_str(&format!(
r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#, format!("print_string_{}", print_strings.len()), format!("print_string_{}", print_strings.len()), print_string.length
                ));
                print_strings.push(print_string.value);
            },
            Token::PrintVariable(name) => {
                let variable = variables.get(&name);

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

"#, name, name, format!("{}_end", name), format!("{}_end", name)));
                            },
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

"#, name, name, format!("{}_end", name), format!("{}_end", name)));                            
                            },
                            // Return if not string.
                            _ => {
                                return Err(String::from("variable is not a string!!"));
                            }
                        }
                    }
                    None => {
                        return Err(String::from("no variable found"))
                    }
                };
            },
            Token::String(string) => {
                // Insert string variable to variables hashmap.
                variables.insert(string.name.clone(), VariableType::String(string));
            },
            Token::Error(err) => {
                // Thow an error and return.
                return Err(err);
            },
            Token::EOF => {
                // Break if it is end of file.
                println!("Completed!!!");
                break;
            },
            Token::Comment => {
                // Do nothing if it is comment
            }
        }
    }

    return Ok(parsed_text);
}
