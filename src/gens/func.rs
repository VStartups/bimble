use std::process::exit;

use crate::{
    gens::{pin::pin, print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};

pub fn p_fn(
    code: &str,
    vrs: &mut Vec<(String, i32)>,
    index: &mut i32,
) -> (String, TokenList, Vec<String>, Vec<(String, i32)>) {
    println!("Function process called...");
    let code: Vec<&str> = code.split("\n").collect();
    let mut flist: Vec<String> = Vec::new();
    let mut infn = false;
    let mut fnm = String::new();
    let mut tl = TokenList::new();
    let mut function_body = TokenList::new(); // TokenList for the function's content
    let mut undefined_fn_calls: Vec<(String, usize, String)> = Vec::new(); // Track undefined function calls

    for ln in &code {
        let ln = ln.trim();
        println!("ln : {:?}",ln);
        if ln.starts_with("ON ") && ln.ends_with("{") {
            fnm = optimize_trim(ln);

            if flist.contains(&fnm) {
                eprintln!(
                    "Error: Function '{}' is already defined. Code block: {}",
                    fnm,
                    &code.join("\n")
                );
                exit(1);
            }

            flist.push(fnm.clone());
            println!("Function list inside func.rs: {:?}", flist);
            infn = true;
        } else if infn {
            if ln.trim() == "}" {
                infn = false;
                let func_token = Tokens::Func(fnm.clone(), function_body.clone());
                tl.push(func_token);
            } else if ln.starts_with("echoln(\"") && ln.ends_with("\")") {
                let ptxt = p_print(ln, &tl);
                function_body.push(Tokens::Print(ptxt.clone()));
            } else if ln.starts_with("may ") {
                let (name, var, usename) = pvar(ln, vrs);
                function_body.push(Tokens::Variable(name.clone(), var.clone(), usename.clone()));
            } else if ln.starts_with("takein(") && ln.ends_with(")") {
                let g = pin(ln, &tl);
                if !g.0 {
                    eprintln!(
                        "Error: Variable '{}' used in takein statement is not defined. Line: {}",
                        g.1, ln
                    );
                    exit(1);
                }
                function_body.push(Tokens::Takein(g.1.clone()));
            }
        } else {
            let trimmed_line = ln.trim();
            if !trimmed_line.is_empty() {
                let fn_name = trimmed_line.split('(').next().unwrap_or("").to_string(); // Extract function name up to the first '('
                let mut found = false;
                for i in &flist {
                    if *i == fn_name {
                        tl.push(Tokens::FnCall(fn_name.clone()));
                        found = true;
                        break;
                    }
                }
                if !found {
                    undefined_fn_calls.push((fn_name, *index as usize, ln.to_string()));
                }
            }
        }

        *index += 1;
    }

    // After processing all lines, handle undefined function calls
    let mut notfound = false;
    for (fn_name, line_index, full_line) in undefined_fn_calls {
        if !flist.contains(&fn_name) {
            eprintln!(
                "Error: Undefined function call '{}' at line {}: '{}'",
                fn_name, line_index, full_line
            );
            notfound = true;
        }
    }

    if notfound {
        exit(1);
    }
    println!("Function name: {:?} and code: {:?}", fnm, tl);
    (fnm, tl, flist, vrs.to_vec()) // Return the vrs vector as well
}
fn optimize_trim(s: &str) -> String {
    let mut function_name = String::new();
    let mut inside_parens = false;

    for c in s.chars() {
        if c == '(' {
            inside_parens = true;
        } else if c == ')' {
            inside_parens = false;
        } else if !inside_parens && c != ' ' {
            if function_name != "ON" {
                if c != '{' {
                    function_name.push(c);
                }
            } else if function_name == "ON" {
                function_name.clear();

                if c != '{' {
                    function_name.push(c);
                }
            }
        }
    }

    function_name.trim().to_string()
}
