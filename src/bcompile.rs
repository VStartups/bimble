use indicatif::ProgressBar;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::{exit, Command},
    time::Duration,
};

use crate::tokens::{TokenList, Tokens, Var};

// Generates general C code including the main function and headers
pub fn gen_cc(tokens: TokenList) -> String {
    let mut var_map = HashMap::new();
    let includes = "#include <stdio.h>\n#include <string.h>\n".to_string();
    let mut func_defs = String::new();
    let mut main_code = String::new();
    let mut current_func = None;

    for token in tokens.get() {
        match token {
            Tokens::Variable(name, ty, use_name) => {
                let declaration = match ty {
                    Var::STR(txt) => format!("char {}[{}] = \"{}\";\n", use_name, use_name.len() + 100, txt),
                    Var::F(f) => format!("double {} = {};\n", use_name, f),
                    Var::INT(i) => format!("long long {} = {};\n", use_name, i),
                };
                var_map.insert(name.to_string(), declaration.clone());
                if current_func.is_none() {
                    main_code.push_str(&declaration);
                }
            }
            Tokens::Func(name, code) => {
                let nested_code = gen_fc(code.clone());
                let func_code = format!("void {}(){{\n{}\n}}\n", name, nested_code);
                func_defs.push_str(&func_code);

                if current_func.is_none() {
                    current_func = Some(name.clone());
                }
            }
            Tokens::FnCall(name) => {
                if let Some(ref func_name) = current_func {
                    if func_name == name {
                        main_code.push_str(format!("{}();\n", name).as_str());
                    }
                }
            }
            Tokens::Print(text) => {
                let print_code = generate_print_code(&text, &var_map);
                if let Some(_) = current_func {
                    func_defs.push_str(&print_code);
                } else {
                    main_code.push_str(&print_code);
                }
            }
            Tokens::Takein(name) => {
                let input_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    name, name, name, name
                );
                if let Some(_) = current_func {
                    func_defs.push_str(&input_code);
                } else {
                    main_code.push_str(&input_code);
                }
            }
        }
    }

    format!(
        "{}\n{}\nint main(){{\n{}\nreturn 0;\n}}",
        includes, func_defs, main_code
    )
}

// Generates C code for functions only
pub fn gen_fc(tokens: TokenList) -> String {
    let mut func_code = String::new();
    let mut current_func = None;

    for token in tokens.get() {
        match token {
            Tokens::Variable(_name, ty, use_name) => {
                let declaration = match ty {
                    Var::STR(txt) => format!("char {}[{}] = \"{}\";\n", use_name, use_name.len() + 100, txt),
                    Var::F(f) => format!("double {} = {};\n", use_name, f),
                    Var::INT(i) => format!("long long {} = {};\n", use_name, i),
                };
                func_code.push_str(&declaration);
            }
            Tokens::Func(name, code) => {
                let nested_code = gen_fc(code.clone());
                let func_code_snippet = format!("void {}(){{\n{}\n}}\n", name, nested_code);
                func_code.push_str(&func_code_snippet);

                if current_func.is_none() {
                    current_func = Some(name.clone());
                }
            }
            Tokens::FnCall(name) => {
                if let Some(ref func_name) = current_func {
                    if func_name == name {
                        func_code.push_str(format!("{}();\n", name).as_str());
                    }
                }
            }
            Tokens::Print(text) => {
                let print_code = generate_print_code(&text, &HashMap::new()); // No vars in function-only code
                func_code.push_str(&print_code);
            }
            Tokens::Takein(name) => {
                let input_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    name, name, name, name
                );
                func_code.push_str(&input_code);
            }
        }
    }

    func_code
}

fn generate_print_code(text: &str, var_map: &HashMap<String, String>) -> String {
    let mut output = String::new();
    let mut vars = Vec::new();
    output.push_str("printf(\"");

    let mut prev_was_var = false;
    for word in text.split_whitespace() {
        if word.starts_with('$') {
            let var_name = word.strip_prefix("$").unwrap();
            if let Some(var_decl) = var_map.get(var_name) {
                if var_decl.contains("long long") {
                    vars.push(var_name.to_string());
                    output.push_str(" %lld");
                } else if var_decl.contains("double") {
                    vars.push(" %f".to_string());
                    vars.push(var_name.to_string());
                } else if var_decl.contains("char") {
                    output.push_str(" %s");
                    vars.push(var_name.to_string());
                }
                prev_was_var = true;
            }
        } else {
            if prev_was_var || !output.ends_with('"') {
                output.push(' ');
            }
            output.push_str(word);
            prev_was_var = false;
        }
    }

    output.push_str("\\n\"");
    if !vars.is_empty() {
        output.push_str(", ");
        output.push_str(&vars.join(", "));
    }
    output.push_str(");\n");

    output
}

pub fn bc_gcc(tokens: TokenList) {
    check_compiler("gcc");
    compile_code(tokens, "gcc");
}

pub fn bc_clang(tokens: TokenList) {
    check_compiler("clang");
    compile_code(tokens, "clang");
}

fn check_compiler(compiler: &str) {
    let output = Command::new(compiler)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("{} is not installed or not found in the system's PATH.", compiler);
            exit(1);
        });

    if !output.status.success() {
        eprintln!("{} is installed, but there was an issue running the command.", compiler);
        exit(1);
    }
}

fn compile_code(tokens: TokenList, compiler: &str) {
    let code = gen_cc(tokens);
    let mut file = File::create("t.c").unwrap_or_else(|_| {
        eprintln!("Unable to create 't.c'. Compilation failed.");
        exit(1);
    });

    if let Err(e) = file.write_all(code.as_bytes()) {
        eprintln!("Failed to write C code to 't.c': {}", e);
        exit(1);
    }

    compile_with(compiler);
}

fn compile_with(compiler: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_message("Compiling...");

    let compile_output = Command::new(compiler)
        .arg("-c")
        .arg("t.c")
        .arg("-o")
        .arg("t.o")
        .arg("-O3")
        .arg("-static")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute {}.", compiler);
            exit(1);
        });

    pb.enable_steady_tick(Duration::from_millis(50));
    while !compile_output.status.success() {
        std::thread::sleep(Duration::from_millis(100));
    }
    pb.finish_with_message("Compilation complete.");

    if !compile_output.status.success() {
        eprintln!(
            "Compilation failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&compile_output.stderr)
        );
        exit(1);
    }

    let link_output = Command::new(compiler)
        .arg("t.o")
        .arg("-o")
        .arg("t")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute {} for linking.", compiler);
            exit(1);
        });

    pb.set_message("Linking...");
    pb.enable_steady_tick(Duration::from_millis(50));
    while !link_output.status.success() {
        std::thread::sleep(Duration::from_millis(100));
    }
    pb.finish_with_message("Linking complete.");

    if !link_output.status.success() {
        eprintln!(
            "Linking failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&link_output.stderr)
        );
        exit(1);
    }

    fs::remove_file("t.o").expect("Failed to remove 't.o'");
}
