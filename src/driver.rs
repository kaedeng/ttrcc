use std::env;
use std::process::Command;

use lexer::lex;

#[path="./lexer.rs"]
pub mod lexer;

#[cfg(test)]
#[path="./unit_tests/driver_test.rs"]
mod driver_test;

enum Args {
    Lex,
    Parse,
    Codegen,
    Default
}

#[allow(unused_assignments)]
pub fn driver(){
    let mut run_arg = Args::Default;

    if env::args().len() < 2 || env::args().len() > 3 {
        panic!("Incorrect Number of Arguments")
    }
    else if env::args().len() == 3 {
        let args: Vec<String> = env::args().collect();
        match args[2].as_str() {
            "--lex" => {
                run_arg = Args::Lex;
            }
            "--parse" => {
                run_arg = Args::Parse;
            }
            "--codegen" => {
                run_arg = Args::Codegen;
            }
            _ => {
                panic!("Unknown argument")
            }
        }
    }
    // Reminder to delete proprocess file after compiling
    let input_file = env::args().nth(1).expect("Input file not provided!");
    let ppced_file = input_file.replace(".c", ".i");

    let result = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(&input_file)
        .arg("-o")
        .arg(&ppced_file)
        .output()
        .expect("Failed to execute gcc.");
    
    if !result.status.success() {
        eprintln!("gcc failed with status: {}", result.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    } else {
        println!("gcc ran successfully. Preprocessed file saved to {}", ppced_file);
    }

    
    match run_arg {
        Args::Default => {
            println!("TODO: Implement This!");
            assemble(&input_file.as_str());
        }
        Args::Lex => {
            lex(&ppced_file);
        }
        Args::Parse => {
            println!("TODO: Implement This!");
        }
        Args::Codegen => {
            println!("TODO: Implement This!");
        }
    }
        
}

fn assemble(input_file: &str) {
    let asm_file = "todo";
    let output_file = input_file.replace(".c", "");
    let result = Command::new("gcc")
        .arg(&asm_file)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("Failed to execute gcc.");

    if !result.status.success() {
        eprintln!("gcc failed with status: {}", result.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    } else {
        println!("gcc ran successfully. Assembly file saved to {}", output_file);
    }
}