use crate::tokens::{TokenList, Tokens, Var};
use indicatif::ProgressBar;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    process::{exit, Command},
    time::Duration,
};

pub fn gen_cc(tokens: TokenList) -> String {
    let mut vars = Vec::new();
    let mut var_names_count = HashMap::new();
    let includes = "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n".to_string();
    let mut func_defs = String::new();
    let mut main_code = String::new();
    let mut updated_tokens = Vec::new();

    for token in tokens.get() {
        match token {
            Tokens::Variable(name, ty, use_name) => {
                let unique_use_name = get_unique_use_name(&use_name, &mut var_names_count);
                updated_tokens.push(Tokens::Variable(
                    name.clone(),
                    ty.clone(),
                    unique_use_name.clone(),
                ));

                vars.push((name.clone(), ty.clone(), unique_use_name.clone()));

                let declaration = match ty {
                    Var::STR(s) => format!(
                        "char {}[{}] = \"{}\";\n",
                        unique_use_name,
                        unique_use_name.len() + 100,
                        s
                    ),
                    Var::F(f) => format!("double {} = {};\n", unique_use_name, f),
                    Var::INT(i) => format!("long long {} = {};\n", unique_use_name, i),
                };
                main_code.push_str(&declaration);
            }
            Tokens::Func(name, code) => {
                let nested_code = gen_fc(code.clone(), &vars, &mut var_names_count);
                func_defs.push_str(&format!("void {}(){{\n{}\n}}\n", name, nested_code));
            }
            Tokens::FnCall(name) => {
                main_code.push_str(&format!("{}();\n", name));
            }
            Tokens::Print(text) => {
                let print_code = generate_print_code(&text, &vars);
                main_code.push_str(&print_code);
            }
            Tokens::Takein(name) => {
                let input_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    name, name, name, name
                );
                main_code.push_str(&input_code);
            }
        }
    }

    let code = format!(
        "{}\n{}\nint main(){{\n{}\nreturn 0;\n}}",
        includes, func_defs, main_code
    );
    code
}
#[allow(unused)]
pub fn gen_fc(
    tokens: TokenList,
    vars: &[(String, Var, String)],
    var_names_count: &mut HashMap<String, usize>,
) -> String {
    let mut func_code = String::new();
    let mut updated_vars: Vec<(String, Var, String)> = Vec::new();
    let mut declvrs = Vec::new();

    for token in tokens.get() {
        match token {
            Tokens::Variable(name, ty, use_name) => {
                if declvrs.contains(use_name) {
                    func_code.push_str(format!("\nfree({});", use_name).as_str());
                }

                let unique_use_name = get_unique_use_name(&use_name, var_names_count);

                let mut found = false;

                for var in &mut updated_vars {
                    if var.0 == *name {
                        var.2 = unique_use_name.clone();
                        found = true;
                        break;
                    }
                }

                if !found {
                    updated_vars.push((name.clone(), ty.clone(), unique_use_name.clone()));
                }

                let declaration = match ty {
                    Var::STR(s) => format!(
                        "char {}[{}] = \"{}\";\n",
                        unique_use_name,
                        unique_use_name.len() + 100,
                        s
                    ),
                    Var::F(f) => format!("double {} = {};\n", unique_use_name, f),
                    Var::INT(i) => format!("long long {} = {};\n", unique_use_name, i),
                };
                func_code.push_str(&declaration);
                declvrs.push(use_name.to_string());
            }
            Tokens::FnCall(name) => {
                func_code.push_str(&format!("{}();\n", name));
            }
            Tokens::Print(text) => {
                let print_code = generate_print_code(&text, &updated_vars);
                func_code.push_str(&print_code);
            }
            Tokens::Takein(name) => {
                let input_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    name, name, name, name
                );
                func_code.push_str(&input_code);
            }
            _ => continue,
        }
    }

    func_code
}

fn get_unique_use_name(base_name: &str, names_count: &mut HashMap<String, usize>) -> String {
    let count = names_count.entry(base_name.to_string()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base_name.to_string()
    } else {
        format!("{}_{}", base_name, *count - 1)
    }
}

const SPECIAL_SYMBOLS: [char; 33] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '{', '}', '[', ']', ':',
    ';', '"', '\'', '<', '>', ',', '.', '/', '?', '|', '\\', '~', '`', ' ',
];

fn generate_print_code(text: &str, vars: &[(String, Var, String)]) -> String {
    let mut output = String::new();
    output.push_str("printf(\"");
    let mut inv = false;
    let mut vrwrd = String::new();
    let mut vrs = Vec::new();
    for i in text.chars() {
        if i == '$' && !inv {
            inv = true;
        } else if inv && !SPECIAL_SYMBOLS.contains(&i) {
            vrwrd.push(i);
        } else if inv && SPECIAL_SYMBOLS.contains(&i) {
            inv = false;
            for v in vars {
                if v.0 == vrwrd {
                    vrs.push(v.2.clone());
                    match v.1 {
                        Var::INT(_) => {
                            output.push_str(format!("{}%lld", i).as_str());
                        }
                        Var::STR(_) => {
                            output.push_str(format!("{}%s", i).as_str());
                        }
                        Var::F(_) => {
                            output.push_str(format!("{}%f", i).as_str());
                        }
                    }
                }
            }
        } else {
            if !inv {
                output.push(i);
            }
        }
    }
    for v in vars {
        if v.0 == vrwrd {
            vrs.push(v.2.clone());
            match v.1 {
                Var::INT(_) => {
                    output.push_str(format!("%lld").as_str());
                }
                Var::STR(_) => {
                    output.push_str(format!("%s").as_str());
                }
                Var::F(_) => {
                    output.push_str(format!("%f").as_str());
                }
            }
        }
    }
    if !vrs.is_empty(){

        output.push_str(format!("\\n\",{});", vrs.join(",")).as_str());
    }
    else{
        output.push_str(format!("\\n\");").as_str());
    }
    output
}

pub fn check_compiler(compiler: &str) {
    let output = Command::new(compiler)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| {
            eprintln!("{} is not installed or not found in the system's PATH.", compiler);
            exit(1);
        });

    if !output.status.success() {
        eprintln!("{} is not installed or not found in the system's PATH.", compiler);
        exit(1);
    }
}

pub fn show_progress_bar() {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Compiling...");
    std::thread::sleep(Duration::from_secs(2));
    pb.finish_with_message("Compilation finished.");
}

pub fn compile_code(tokens: TokenList, compiler: &str) {
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

pub fn compile_with(compiler: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_message("Compiling...");

    let compile_output = Command::new(compiler)
        .arg("-c")
        .arg("t.c")
        .arg("-o")
        .arg("t.o")
        .arg("-O3")
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
    
    if !link_output.status.success() {
        eprintln!(
            "Linking failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&link_output.stderr)
        );
        exit(1);
    }
    pb.finish_with_message("Finished building");

    fs::remove_file("t.o").unwrap_or_else(|_| {
        eprintln!("Failed to delete temporary file 't.o'.");
        exit(1);
    });
}
