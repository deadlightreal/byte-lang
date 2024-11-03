use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{fs::File, io::Read};
use unzip::Unzipper;

mod compile_asm;
mod datatypes;

use compile_asm::compile_asm;
use datatypes::parser::{get_offset, parse_code};
use datatypes::{FunctionStruct, ArgType, StackItem, LoopStruct, VariableType, StackFrame, DataBoolean, DataNumber};

fn main() {
    let start = std::time::Instant::now();

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
    };

    println!("{:?}", start.elapsed());
}

fn install_dependency() {
    let dependency_name = std::env::args().nth(2).unwrap();
    let project_dir = get_project_folder().unwrap();
    let dependencies_dir = project_dir.join("dependencies");

    let version : String = match std::env::args().nth(3) {
        Some(version) => version,
        None => String::new()
    };

    let response = reqwest::blocking::get(format!("http://localhost:8080/installPackage?package={}&version={}", dependency_name, version)).unwrap();

    match response.status().as_u16() {
        200 => {
            let before_items = std::fs::read_dir(dependencies_dir.clone()).unwrap();

            let mut items_hashmap : HashMap<String, u8> = HashMap::new();

            for item in before_items {
                items_hashmap.insert(item.unwrap().path().to_str().unwrap().to_string(), 0);
            }

            let mut dependency_dir = dependencies_dir.clone();
            dependency_dir.push(format!("{}.zip", dependency_name));

            let mut dest = File::create(dependency_dir.clone()).unwrap();

            let content = response.bytes().unwrap();

            std::io::copy(&mut content.as_ref(), &mut dest).unwrap();

            let file = File::open(dependency_dir.clone()).unwrap();

            Unzipper::new(file, &dependencies_dir.as_path()).unzip().unwrap();

            std::fs::remove_file(dependency_dir.clone()).unwrap();

            let after_items = std::fs::read_dir(dependencies_dir.clone()).unwrap();

            for item in after_items {
                let dir : String = item.unwrap().path().to_str().unwrap().to_string();
                let map_item = items_hashmap.get(&dir);
                match map_item {
                    Some(_) => {},
                    None => {
                        let new_dir = dependencies_dir.clone().join(dependency_name.clone());
                        std::fs::rename(dir, new_dir).unwrap();
                    }
                };
            }
        },
        409 => {
            println!("Error: {}", response.text().unwrap());
        },
        _ => {

        }
    }

    
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
.text
_start:

"#
    )
    .expect("Error Writing File");

    let parsed_text = parse_code(
        &content as &str,
    );

    match parsed_text {
        Ok(text) => write!(writer, "{}", text).expect("error writing to a file"),
        Err(err) => {
            println!("{}", err);
            return;
        }
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
