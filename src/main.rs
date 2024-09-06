use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Read};

mod compile_asm;
mod datatypes;

use compile_asm::compile_asm;
use datatypes::parser::{get_offset, parse_code};
use datatypes::{FunctionStruct, StackItem, LoopStruct, Token, Tokenizer, VariableType, StackFrame};

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

    let parsed_text = parse_code(
        &content as &str,
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
        let got_offset = get_offset(stack.clone());
        stack.push(StackFrame{stack_items: HashMap::new()});
        stack.last_mut().unwrap().stack_items.insert(String::from("stack-pointer"), StackItem{ size: 16, offset: got_offset, variable: VariableType::Return() });
        let text = parse_code(
            &func.content as &str,
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

    add sp, sp, #{}

    ldr X30, [sp]

    ret
"#,
            func.name, text, current_offset as u32 - got_offset
            )
        .expect("error writing to a file");
        stack.pop();
        println!("{:?}", stack);
    }

    let mut index = 0;

    for item in labels {
        write!(writer, "{}", item).expect("error writing to a file");
    }

    write!(writer, ".data\n").expect("error writing to a file");

    // Create a new_line string that contains \n.
    write!(writer, "new_line: .ascii \"\\n\"\n")
        .expect("Error Writing to a file");

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
