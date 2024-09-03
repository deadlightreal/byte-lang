use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Read};

mod compile_asm;
mod datatypes;

use compile_asm::compile_asm;
use datatypes::{CompareType, FunctionStruct, StackItem, LoopStruct, Token, Tokenizer, VariableType, StackFrame};

fn main() {
    let command = std::env::args().nth(1).expect("Please Provide a command");
    match command.as_str() {
        "run" => {
            run_file();
        }
        _ => {}
    }
}

fn run_file() {
    // Getting second arg that should provide location of file that they want to run.
    let file_location = std::env::args()
        .nth(2)
        .expect("Please Provide File Location");

    // Open the file.
    let mut file = File::open(file_location).expect("Error Oppening File");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Error Reading As String");

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
    write!(
        writer,
        r#".global _start
.align 2
_start:

"#
    )
    .expect("Error Writing File");

    // Init tokenizer.
    let tokenizer = Tokenizer::new(&content);

    // Store strings used to print.
    let mut print_strings: Vec<String> = Vec::new();

    let mut current_offset : u64 = 0;

    let mut labels : Vec<String> = Vec::new();

    // Variables like numbers and strings.
    let mut stack : Vec<StackFrame> = Vec::new();
    stack.push(StackFrame{stack_items: HashMap::new()});

    let mut functions: HashMap<String, FunctionStruct> = HashMap::new();

    // Store number of loops made.
    let loop_number: u32 = 0;
    let mut compare_number: u32 = 0;

    let mut loops: Vec<LoopStruct> = Vec::new();

    let parsed_text = handle_parsing(
        tokenizer,
        &mut stack,
        loop_number,
        &mut print_strings,
        &mut loops,
        &mut compare_number,
        &mut functions,
        &mut labels,
        &mut current_offset,
    );

    match parsed_text {
        Ok(text) => write!(writer, "{}", text).expect("error writing to a file"),
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    for func in functions.clone().values() {
        let tokenizer = Tokenizer::new(&func.content as &str);
        let got_offset = get_offset(stack.clone());
        stack.push(StackFrame{stack_items: HashMap::new()});
        stack.last_mut().unwrap().stack_items.insert(String::from("return"), StackItem{ size: 16, offset: got_offset, variable: VariableType::Return() });
        let text = handle_parsing(tokenizer,
            &mut stack,
            loop_number,
            &mut print_strings,
            &mut loops,
            &mut compare_number,
            &mut functions,
            &mut labels,
            &mut current_offset,
        ).unwrap();
        current_offset += 16;
        write!(
            writer,
            r#"f_{}:
    str X30, [sp]
    sub sp, sp, #16

{}

    ldr X30, [sp, #{}]
    ret
"#,
            func.name, text, current_offset as u32 - got_offset
        )
        .expect("error writing to a file");
        println!("{:?}", stack);
    }

    let mut index = 0;

    for item in labels {
        write!(writer, "{}", item).expect("error writing to a file");
    }

    write!(writer, ".data\n").expect("error writing to a file");

    // Create a new_line string that contains \n.
    write!(writer, "new_line: .ascii \"\\n\"\nfn_end: .quad _start\n")
        .expect("Error Writing to a file");

    let mut loop_index = 0;
    for loop_struct in loops {
        write!(
            writer,
            "l_{}_limit: .word {}\nl_{}_index: .word 0\nl_{}_return: .quad 0\n",
            loop_index, loop_struct.limit, loop_index, loop_index
        )
        .expect("error writing to a file");
        loop_index += 1;
    }

    // Insert print strings into data section.
    for print_string in print_strings {
        write!(
            writer,
            "print_string_{}: .ascii \"{}\"\n",
            index, print_string
        )
        .expect("Error writing to a file");
        index += 1;
    }

    /*
    // Write variables at the data section.
    for item in variables.values() {
        // Match the variable type.
        match item {
            VariableType::String(string) => {
                write!(
                    writer,
                    "{}: .asciz \"{}\"\n{}_end:\n{}_length: .word {}\n",
                    string.name.clone(),
                    string.value,
                    string.name,
                    string.name,
                    string.value.len()
                )
                .expect("Error Writing to a file");
            }
            VariableType::Number(number) => {
                write!(writer, "{}: .word {}\n", number.name, number.value)
                    .expect("error writing to file");
            }
            VariableType::Bool(bool) => {
                let val: u8 = match bool.value {
                    false => 0,
                    true => 1,
                };
                write!(writer, "{}: .byte {}\n", bool.name, val).expect("error writing to file");
            }
        }
    }
    */

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

fn handle_parsing(
    mut tokenizer: Tokenizer,
    stack: &mut Vec<StackFrame>,
    mut loop_number: u32,
    print_strings: &mut Vec<String>,
    loops: &mut Vec<LoopStruct>,
    compare_number: &mut u32,
    functions: &mut HashMap<String, FunctionStruct>,
    labels: &mut Vec<String>,
    current_offset: &mut u64,
    ) -> Result<String, String> {
    let mut parsed_text = String::new();

    loop {
        let token: Token = tokenizer.next_token(stack.clone(), functions.clone());

        match token {
            Token::Asm(asm) => {
                println!("asm");
                parsed_text.push_str(&asm);
            },
            Token::Import(file_location) => {
                let mut file = File::open(file_location).expect("error opening file");

                let mut file_content = String::new();

                file.read_to_string(&mut file_content).expect("Error reading as string");

                println!("{}", file_content);
                
                let file_tokenizer = Tokenizer::new(&file_content as &str);

                let parsed_file_code = handle_parsing(file_tokenizer, stack, loop_number, print_strings, loops, compare_number, functions, labels, current_offset);

                match parsed_file_code {
                    Ok(code) => {
                        parsed_text.push_str(&code);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            },
            /*
            Token::DataBoolean(databoolean) => {
                variables.insert(databoolean.name.clone(), VariableType::Bool(databoolean));
            }
            */
            Token::CallFunction(name) => {
                parsed_text.push_str(&format!(
                    r#"    bl f_{}

"#,
                    name
                ));
            }
            Token::Function(func) => {
                functions.insert(
                    func.name.clone(),
                    FunctionStruct {
                        content: func.content,
                        name: func.name,
                    },
                );
            }
            Token::Terminate() => {
                parsed_text.push_str(
                    r#"    mov X0, #0
    mov X16, #1
    svc #0x80

"#,
                );
            }
            Token::Compare(compare_args) => {
                let current_number = compare_number.clone();
                *compare_number += 1;

                let mut index = 0;
                for arg in compare_args.compare_types {
                    match arg {
                        CompareType::Number(num) => {
                            parsed_text.push_str(&format!(
                                r#"    mov W{}, #{}

"#,
                                1 + index,
                                num
                            ));
                        }
                        CompareType::VariableNumber(variable) => {
                            println!("cmp type var");
                            parsed_text.push_str(&format!(
                                r#"
    ldr W{}, [sp, #{}]

"#, 1 + index, variable.offset));
                        }
                        CompareType::None() => {
                            return Err(String::from("Compare type was not given"));
                        }
                    }
                    index += 1;
                }

                parsed_text.push_str(&format!(
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

                    parsed_text.push_str(&format!(
                        r#"    b.{} {}_{}

"#,
                        shortcut, compare_type, current_number
                    ));
                }

                parsed_text.push_str(&format!(
                    r#"    bl continue_{}
"#,
                    current_number
                ));

                for symbol in compare_args.symbols {
                    let new_tokenizer = Tokenizer::new(&symbol.function_content as &str);
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
                    let parsed_compare_text = handle_parsing(
                        new_tokenizer,
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
                            parsed_text.push_str(&format!(
                                r#"{}_{}:
{}

    bl continue_{}
"#,
                                symbol_type, current_number, content, current_number
                            ));
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    };
                }

                parsed_text.push_str(&format!(
                    r#"

continue_{}:

"#,
                    current_number
                ));
            }
            Token::Number(number) => {
                let ok_offset = get_offset(stack.clone());
                let stack_item : StackItem = StackItem{offset: ok_offset, size: 16, variable: VariableType::Number(number.clone())};
             
                *current_offset += 16;

                parsed_text.push_str(&format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16

"#, number.value));
                let last = stack.last_mut().expect("error getting last mut from stack");
                last.stack_items.insert(number.name, stack_item);
                println!("{:?}", stack);
            }
            Token::Loop(loop_token) => {
                let num = loop_number;
                loop_number += 1;
                loops.push(LoopStruct {
                    limit: loop_token.number as u32,
                });
                let new_tokenizer = Tokenizer::new(&loop_token.content as &str);
                let compiled_content = handle_parsing(
                    new_tokenizer,
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
                        parsed_text.push_str(&format!(
                            r#"    bl l_{}_start

"#,
                            num
                        ));
                        labels.push(format!(r#"
l_{}_start:
    adrp X19, l_{}_return@PAGE
    add X19, X19, l_{}_return@PAGEOFF

    str X30, [X19]

    b l_{}

l_{}:

{}

    adrp X13, l_{}_index@PAGE   
    add X13, X13, l_{}_index@PAGEOFF
    ldr W11, [X13]
    add W11, W11, #1
    str W11, [X13]

    adrp X14, l_{}_limit@PAGE
    add X14, X14, l_{}_limit@PAGEOFF
    ldr W12, [X14]

    cmp W12, W11
    b.ne l_{}

    mov W15, #0
    str W15, [X13]

    adrp X19, l_{}_return@PAGE
    add X19, X19, l_{}_return@PAGEOFF

    ldr X30, [X19]

    ret

"#, num, num, num, num, num, content, num, num, num, num, num, num, num))
                    }
                    Err(err) => return Err(err),
                }
            }
            Token::WaitNumber(number) => {
                let seconds = number.floor();
                //let nanoseconds = (number - seconds) * 1000000000.0;
                // For future devs: please add a feature to wait .. of nanoseconds.

                // Wait .. seconds and then continue to execute code.
                parsed_text.push_str(&format!(
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
                parsed_text.push_str(&format!(
                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#,
                    format!("print_string_{}", print_strings.len()),
                    format!("print_string_{}", print_strings.len()),
                    print_string.length + 1
                ));
                print_strings.push(print_string.value);
            }
            Token::PrintString(print_string) => {
                // Same as printlnString just don't add \n to print_string.
                parsed_text.push_str(&format!(
                    r#"    mov X0, #1
    adrp X1, {}@PAGE
    add X1, X1, {}@PAGEOFF
    mov X2, {}
    mov X16, #4
    svc #0x80

"#,
                    format!("print_string_{}", print_strings.len()),
                    format!("print_string_{}", print_strings.len()),
                    print_string.length
                ));
                print_strings.push(print_string.value);
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
                return Err(err);
            }
            Token::EOF => {
                // Break if it is end of file.
                println!("Completed!!!");
                break;
            }
            Token::Comment => {
                // Do nothing if it is comment
            }
            _ => {}
        }
    }

    return Ok(parsed_text);
}

fn get_offset(stack : Vec<StackFrame>) -> u32 {
    let last_stack = stack.last().expect("error getting stack");

    let mut biggest_offset = 0;

    for item in last_stack.stack_items.values() {
        if item.offset >= biggest_offset {
            biggest_offset = item.offset + item.size;
        }
    }

    if biggest_offset == 0 {
        if stack.len() >= 2 {
            let stack_before = stack.get(stack.len() - 1).unwrap();

            for item in stack_before.stack_items.values() {
                if item.offset > biggest_offset {
                    biggest_offset = item.offset + item.size;
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
