use std::path::PathBuf;
use std::process::Command;
use std::fs;

pub fn compile_asm(current_dir : PathBuf) {
    // Create paths of output assembly file, output file and final output file.
    let mut assembly_file = PathBuf::from(current_dir.clone());
    assembly_file.push("output.s");

    let mut output_file = PathBuf::from(current_dir.clone());
    output_file.push("output.o");

    let mut final_file = PathBuf::from(current_dir.clone());
    final_file.push("output");

    let mut new_file_location = PathBuf::from(current_dir);
    new_file_location.push("output.s");

    // Execute the compiling commands
    let status = Command::new("as")
                        .arg("-o")
                        .arg(output_file.clone())
                        .arg(new_file_location)
                        .status()
                        .expect("Failed to execute command");

    if status.success() == false {
        println!("Failed to compile assembly file to output");
        return;
    }

    let sdk_path_output = Command::new("xcrun")
        .arg("-sdk")
        .arg("macosx")
        .arg("--show-sdk-path")
        .output()
        .expect("Failed to execute xcrun");

    // Convert the output to a string
    let sdk_path = String::from_utf8(sdk_path_output.stdout)
        .expect("Failed to convert SDK path to string")
        .trim()
        .to_string();

    let status = Command::new("ld")
        .arg("-macos_version_min")
        .arg("11.0.0")
        .arg("-o")
        .arg("output")
        .arg("output.o")
        .arg("-lSystem")
        .arg("-syslibroot")
        .arg(sdk_path)
        .arg("-e")
        .arg("_start")
        .arg("-arch")
        .arg("arm64")
        .status()
        .expect("Failed to execute ld command");
    
    if status.success() == false {
        println!("Failed to compile output file to final");
        return;
    }

    fs::remove_file(assembly_file).expect("Error Removing Assembly File");

    // Remove output file.
    fs::remove_file(output_file).expect("Error removing output file");
}
