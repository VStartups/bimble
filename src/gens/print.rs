use crate::tokens::{TokenList, Tokens};
use std::process::exit;

pub fn p_print(code: &str, tl: &TokenList) -> String {
    if !code.starts_with("echoln(\"") || !code.ends_with("\")") {
        eprintln!("Error: Invalid format. Code: '{}'", code);
        exit(1);
    }

    let txt = &code["echoln(\"".len()..code.len() - 2];
    let mut result = String::with_capacity(txt.len()); // Preallocate capacity
    let mut wrd = String::new();
    let mut inside_var = false;

    for c in txt.chars() {
        match c {
            '$' => {
                if inside_var {
                    eprintln!(
                        "Error: Unexpected '$' inside variable mode. Code: '{}'",
                        code
                    );
                    exit(1);
                }
                if !wrd.is_empty() {
                    process_variable(&wrd, tl, &mut result, code);
                    wrd.clear();
                }
                inside_var = true;
            }
            ' ' | ';' | ',' | '.' | ':' => {
                if inside_var {
                    process_variable(&wrd, tl, &mut result, code);
                    wrd.clear();
                    inside_var = false;
                }
                result.push(c);
            }
            _ if inside_var => {
                wrd.push(c);
            }
            _ => {
                if inside_var {
                    process_variable(&wrd, tl, &mut result, code);
                    wrd.clear();
                    inside_var = false;
                }
                result.push(c);
            }
        }
    }

    if inside_var {
        process_variable(&wrd, tl, &mut result, code);
    }

    result
}

fn process_variable(wrd: &str, tl: &TokenList, result: &mut String, code: &str) {
    if tl.get().iter().any(|token| {
        if let Tokens::Variable(name, _, _) = token {
            *name == wrd
        } else {
            false
        }
    }) {
        result.push_str(&format!("${} ", wrd));
    } else {
        eprintln!("Variable '{}' not found. Code: \n{}", wrd, code);
        exit(1);
    }
}
