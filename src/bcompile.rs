use indicatif::ProgressBar;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::{exit, Command},
    time::Duration,
};

use crate::tokens::{TokenList, Tokens, Var};

// Function to generate C code based on tokens
pub fn gen_cc(tokens: TokenList, forf: bool) -> String {
    let mut var_map: HashMap<String, String> = HashMap::new();
    let mut cc = String::new(); // Buffer for main code
    let mut ccode = String::new(); // Buffer for function code

    if !forf {
        cc.push_str("#include <stdio.h>\n#include <string.h>\nint main(){\n");
    }

    // Process tokens
    for token in tokens.get() {
        match token {
            Tokens::Variable(nm, ty, usename) => {
                let var_declaration = match ty {
                    Var::STR(txt) => {
                        format!("char {}[{}] = \"{}\";\n", usename, usename.len() + 100, txt)
                    }
                    Var::F(f) => format!("double {} = {};\n", usename, f),
                    Var::INT(i) => format!("long long {} = {};\n", usename, i),
                };
                var_map.insert(nm.to_string(), var_declaration.clone());
                cc.push_str(&var_declaration);
            }
            Tokens::Func(fname, code) => {
                let mut func_code = String::new();
                func_code.push_str(format!("void {}(){{\n", fname).as_str());
                func_code.push_str(&gen_cc(code.clone(), true)); // Recursive call for function content
                func_code.push_str("\n}\n");
                ccode.push_str(&func_code);
            }
            Tokens::FnCall(nm) => {
                cc.push_str(format!("{}();", nm).as_str());
            }
            Tokens::Print(txt) => {
                let mut output = String::new();
                let mut vars_list = Vec::new();

                output.push_str("printf(\"");

                let mut prev_word_is_var = false;
                for word in txt.split_whitespace() {
                    if word.starts_with('$') {
                        let var_name = word.strip_prefix("$").unwrap();
                        if let Some(var_declaration) = var_map.get(var_name) {
                            if var_declaration.contains("long long") {
                                vars_list.push(var_name.to_string());
                                output.push_str(" %lld");
                            } else if var_declaration.contains("double") {
                                vars_list.push(" %f".to_string());
                                vars_list.push(var_name.to_string());
                            } else if var_declaration.contains("char") {
                                output.push_str(" %s");
                                vars_list.push(var_name.to_string());
                            }
                            prev_word_is_var = true;
                        }
                    } else {
                        if prev_word_is_var || !output.ends_with('"') {
                            output.push(' ');
                        }
                        output.push_str(word);
                        prev_word_is_var = false;
                    }
                }

                output.push_str("\\n\"");

                if !vars_list.is_empty() {
                    output.push_str(", ");
                    output.push_str(&vars_list.join(", "));
                }
                output.push_str(");\n");
                cc.push_str(&output);
            }
            Tokens::Takein(nm) => {
                let takein_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    nm, nm, nm, nm
                );
                cc.push_str(&takein_code);
            }
        }
    }

    if !forf {
        cc.push_str("\nreturn 0;\n}");
    }
    cc.push_str(&ccode);

    cc
}

// Function to compile C code with GCC
pub fn bc_gcc(tokens: TokenList) {
    check_gcc();

    let c_code = gen_cc(tokens, false);
    let mut cfile = File::create("t.c").unwrap_or_else(|_| {
        eprintln!("Unable to create 't.c'. Compilation failed.");
        exit(1);
    });

    if let Err(e) = cfile.write_all(c_code.as_bytes()) {
        eprintln!("Failed to write C code to 't.c': {}", e);
        exit(1);
    }

    compile("gcc");
}

// Function to compile C code with Clang
pub fn bc_clang(tokens: TokenList) {
    check_clang();

    let c_code = gen_cc(tokens, false);
    let mut cfile = File::create("t.c").unwrap_or_else(|_| {
        eprintln!("Unable to create 't.c'. Compilation failed.");
        exit(1);
    });

    if let Err(e) = cfile.write_all(c_code.as_bytes()) {
        eprintln!("Failed to write C code to 't.c': {}", e);
        exit(1);
    }

    compile("clang");
}

// Helper Functions for Compiler Check and Compilation
fn check_gcc() {
    check_compiler("gcc");
}

fn check_clang() {
    check_compiler("clang");
}

fn check_compiler(compiler: &str) {
    let out = Command::new(compiler)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| {
            eprintln!(
                "{} is not installed or not found in the system's PATH.",
                compiler
            );
            exit(1);
        });

    if !out.status.success() {
        eprintln!(
            "{} is installed, but there was an issue running the command.",
            compiler
        );
        exit(1);
    }
}

fn compile(compiler: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_message("Compiling...");

    // Compile the C file to an object file
    let compile_cmd = Command::new(compiler)
        .arg("-c")
        .arg("t.c")
        .arg("-o")
        .arg("t.o") // Output object file
        .arg("-O3") // Maximum optimization
        .arg("-static") // Static linking
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute {}.", compiler);
            exit(1);
        });

    pb.enable_steady_tick(Duration::from_millis(50));
    while !compile_cmd.status.success() {
        std::thread::sleep(Duration::from_millis(100));
    }
    pb.finish_with_message("Compilation complete.");

    if !compile_cmd.status.success() {
        eprintln!(
            "Compilation failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&compile_cmd.stderr)
        );
        exit(1);
    }

    let link_cmd = Command::new(compiler)
        .arg("t.o")
        .arg("-o")
        .arg("t") // Name of the final executable
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute {} for linking.", compiler);
            exit(1);
        });

    pb.set_message("Linking...");
    pb.enable_steady_tick(Duration::from_millis(50));
    while !link_cmd.status.success() {
        std::thread::sleep(Duration::from_millis(100));
    }
    pb.finish_with_message("Linking complete.");

    if !link_cmd.status.success() {
        eprintln!(
            "Linking failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&link_cmd.stderr)
        );
        exit(1);
    }

    println!("Compilation and linking successful. Final executable created.");

    //TODO: Uncomment the lines to Remove the C file after compilation
    // if let Err(e) = fs::remove_file("t.c") {
    //     eprintln!("Failed to delete 't.c': {}", e);
    //     exit(1);
    // }

    // Remove the object file after linking
    if let Err(e) = fs::remove_file("t.o") {
        eprintln!("Failed to delete 't.o': {}", e);
        exit(1);
    }

    println!("Intermediate files 't.c' and 't.o' deleted.");
}
