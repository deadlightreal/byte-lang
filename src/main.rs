use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Read};

mod compile_asm;
mod datatypes;

use compile_asm::compile_asm;
use datatypes::parser::{get_offset, parse_code};
use datatypes::{FunctionStruct, ArgType, StackItem, LoopStruct, VariableType, StackFrame, DataBoolean, DataNumber};

fn main() {
    let command = std::env::args().nth(1).expect("Please Provide a command");
    match command.as_str() {
        "run" => {
            run_file();
        },
        "build" => {
            build_file();            
        },
        "init" => {
            init_command();
        },
        "install" => {
            install_dependency();
        },
        _ => {}
    }
}

fn install_dependency() {
    println!("{:?}", get_project_folder());
}

fn get_project_folder() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().unwrap();

    loop {
        let config_file = dir.join("byte-config.json");
        if config_file.exists() {
            return Ok(dir);
        } else {
            if let Some(dir_parent) = dir.parent() {
                dir = dir_parent.to_path_buf();
            } else {
                return Err(String::from("byte-lang dir not found!!"))
            }
        }
    }
}

fn build_file() {
    compile_file();
    println!("App Compiled \n \n \n--------------------------------------------------------------\n \n \n")
}

fn init_command() {
    let project_name = std::env::args().nth(2).expect("Please provide project name");

    let dir = format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), project_name);

    Command::new("mkdir")
        .arg(dir.clone())
        .status()
        .unwrap();

    Command::new("mkdir")
        .arg(format!("{}/dependencies", dir.clone()))
        .status()
        .unwrap();

    let project_config_location = format!("{}/byte-config.json", dir);

    let project_config_file = File::create(project_config_location).unwrap();

    let mut config_writer = BufWriter::new(project_config_file);

    write!(config_writer,
r#"{{
    "name": "{}",
    "root": "main.byte"
}}
"#, project_name).unwrap();

    let main_file = File::create(format!("{}/main.byte", dir)).unwrap();

    let mut main_file_writer = BufWriter::new(main_file);

    write!(main_file_writer,
r#"\\
Root file
\\

term;"#).unwrap();

    main_file_writer.flush().unwrap();

    config_writer.flush().unwrap();
}

fn compile_file() {
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

    let mut arg_size : u16 = 0;

    for func in functions.clone().values() {
        stack.push(StackFrame{stack_items: HashMap::new()});
        for arg in func.args.clone() {
            let offset = get_offset(stack.clone());
            match arg {
                ArgType::Bool(name) => {
                    stack.last_mut().unwrap().stack_items.insert(name.clone(), StackItem{offset, size: 16, variable: VariableType::Bool(DataBoolean{name, value: false})});
                    current_offset += 16;
                    arg_size += 16;
                },
                ArgType::Number(name) => {
                    stack.last_mut().unwrap().stack_items.insert(name.clone(), StackItem{offset, size: 16, variable: VariableType::Number(DataNumber{name, value: 0})});
                    current_offset += 16;
                    arg_size += 16;
                }
                _ => {},
            };
        };
        let got_offset = get_offset(stack.clone());
        stack.last_mut().unwrap().stack_items.insert(String::from("stack-pointer"), StackItem{ size: 16, offset: got_offset, variable: VariableType::Return() });
        current_offset += 16;
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
        write!(
            writer,
            r#"f_{}:
    str X30, [sp]
    sub sp, sp, #16

{}

    add sp, sp, #{}

    ldr X30, [sp]

    add sp, sp, #{}

    ret
"#,
            func.name, text, current_offset as u32 - got_offset, arg_size
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

}

fn run_file() {
    compile_file();

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
