use std::{
    fs::File,
    io::Write,
    process::{exit, Command},
};

use crate::tokens::{TokenList, Tokens, Var};

// Function to compile with GCC
pub fn bc_gcc(tokens: TokenList) {
    check_gcc();

    let mut cfile = File::create("t.cpp").unwrap_or_else(|_| {
        eprintln!("Unable to create 't.cpp'. Compilation failed.");
        exit(1);
    });

    let mut cc = String::with_capacity(1024);
    cc.push_str("#include <iostream>\n#include <string>\nusing namespace std;\nint main(){");

    for token in tokens.get() {
        match token {
            Tokens::Print(txt) => {
                let segments: Vec<&str> = txt.split_whitespace().collect();
                let mut output = String::with_capacity(1024);

                for (i, segment) in segments.iter().enumerate() {
                    if segment.starts_with('$') {
                        let var_name = &segment[1..];
                        if !output.is_empty() {
                            output.push_str(" << ");
                        }
                        output.push_str(var_name);
                    } else {
                        if !output.is_empty() {
                            output.push_str(" << ");
                        }
                        output.push_str("\"");
                        output.push_str(segment);
                        output.push_str("\"");
                    }

                    if i < segments.len() - 1 {
                        output.push_str(" << \" \"");
                    }
                }

                output.push_str(" << std::endl;");
                cc.push_str(&format!("\nstd::cout << {}", output));
            }

            Tokens::Variable(nm, ty) => {
                match ty {
                    Var::STR(txt) => cc.push_str(&format!("\nstd::string {} = \"{}\";", nm, txt)),
                    Var::F(f) => cc.push_str(&format!("\ndouble {} = {};", nm, f)),
                    Var::INT(i) => cc.push_str(&format!("\nlong long {} = {}LL;", nm, i)),
                }
            }
        }
    }

    cc.push_str("\nreturn 0;\n}");

    if let Err(e) = cfile.write_all(cc.as_bytes()) {
        eprintln!("Failed to write C++ code to 't.cpp': {}", e);
        exit(1);
    }

    compile("g++");
}

// Function to compile with Clang
pub fn bc_clang(tokens: TokenList) {
    check_clang();

    let mut cfile = File::create("t.cpp").unwrap_or_else(|_| {
        eprintln!("Unable to create 't.cpp'. Compilation failed.");
        exit(1);
    });

    let mut cc = String::with_capacity(1024);
    cc.push_str("#include <iostream>\n#include <string>\nusing namespace std;\nint main(){");

    for token in tokens.get() {
        match token {
            Tokens::Print(txt) => {
                let segments: Vec<&str> = txt.split_whitespace().collect();
                let mut output = String::with_capacity(1024);

                for (i, segment) in segments.iter().enumerate() {
                    if segment.starts_with('$') {
                        let var_name = &segment[1..];
                        if !output.is_empty() {
                            output.push_str(" << ");
                        }
                        output.push_str(var_name);
                    } else {
                        if !output.is_empty() {
                            output.push_str(" << ");
                        }
                        output.push_str("\"");
                        output.push_str(segment);
                        output.push_str("\"");
                    }

                    if i < segments.len() - 1 {
                        output.push_str(" << \" \"");
                    }
                }

                output.push_str(" << std::endl;");
                cc.push_str(&format!("\nstd::cout << {}", output));
            }

            Tokens::Variable(nm, ty) => {
                match ty {
                    Var::STR(txt) => cc.push_str(&format!("\nstd::string {} = \"{}\";", nm, txt)),
                    Var::F(f) => cc.push_str(&format!("\ndouble {} = {};", nm, f)),
                    Var::INT(i) => cc.push_str(&format!("\nlong long {} = {}LL;", nm, i)),
                }
            }
        }
    }

    cc.push_str("\nreturn 0;\n}");

    if let Err(e) = cfile.write_all(cc.as_bytes()) {
        eprintln!("Failed to write C++ code to 't.cpp': {}", e);
        exit(1);
    }

    compile("clang++");
}

// Function to check for GCC installation
fn check_gcc() {
    let out = Command::new("g++").arg("--version").output().unwrap_or_else(|_| {
        eprintln!("g++ is not installed or not found in the system's PATH.");
        exit(1);
    });

    if !out.status.success() {
        eprintln!("g++ is installed, but there was an issue running the command.");
        exit(1);
    }
}

// Function to check for Clang installation
fn check_clang() {
    let out = Command::new("clang++").arg("--version").output().unwrap_or_else(|_| {
        eprintln!("clang++ is not installed or not found in the system's PATH.");
        exit(1);
    });

    if !out.status.success() {
        eprintln!("clang++ is installed, but there was an issue running the command.");
        exit(1);
    }
}

// Function to compile the C++ code using a specified compiler
fn compile(compiler: &str) {
    let cmd = Command::new(compiler)
        .arg("t.cpp")
        .arg("-o")
        .arg("t.out")
        .arg("-O3") // Maximum optimization
        .arg("-static") // Static linking for GCC and Clang
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute {}.", compiler);
            exit(1);
        });

    if !cmd.status.success() {
        eprintln!(
            "Compilation failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&cmd.stderr)
        );
        exit(1);
    }

    println!("Compilation successful with {}.", compiler);
}
