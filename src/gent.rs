use std::process::exit;

use crate::{
    gens::{func::p_fn, pin::pin, print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};

pub fn gen_token(code: &str) -> TokenList {
    let mut tl = TokenList::new();
    let mut index = 1;
    let mut vrs: Vec<(String, i32)> = Vec::new();
    let mut flist: Vec<String> = Vec::new();
    let mut fcds = String::new();
    let mut infn = false; // Initialize as false
    let mut undefined_fn_calls: Vec<(String, usize, String)> = Vec::new(); // Store undefined function names with line index and full line

    for line in code.lines() {
        let line = line.trim();

        if line.is_empty() {
            index += 1;
            continue;
        }

        if line.starts_with("echoln(\"") && line.ends_with("\")") && !infn {
            let ptxt = p_print(line, &tl);
            tl.push(Tokens::Print(ptxt.clone()));
        } else if line.starts_with("may ") {
            let (name, var, usename) = pvar(line, &mut vrs);
            tl.push(Tokens::Variable(name.clone(), var.clone(), usename.clone()));
        } else if line.starts_with("takein(") && line.ends_with(")") && !infn {
            let g = pin(line, &tl);
            if !g.0 {
                eprintln!(
                    "Error: Variable '{}' used in takein statement is not defined. Line {}: '{}'",
                    g.1, index, line
                );
                exit(1);
            }
            tl.push(Tokens::Takein(g.1.clone()));
        } else if line.starts_with("ON ") && line.ends_with("{") {
            infn = true;
            fcds.push_str(format!("\n{}", line).as_str());
        } else if infn {
            fcds.push_str(format!("\n{}", line).as_str());
            if line.trim() == "}" {
                infn = false;
                let fpr = p_fn(&fcds, &mut flist, &mut vrs, &mut index);
                flist.push(fpr.0.to_string());
                tl.join_mut(fpr.1);
                fcds.clear(); // Clear function body buffer
            }
        } else {
            // Check for function calls only if the line is not empty
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() {
                let fn_name = trimmed_line.split('(').next().unwrap_or("").to_string(); // Extract function name up to the first '('
                let mut found = false;
                for i in &flist {
                    if *i == format!("{}()", trimmed_line) {
                        tl.push(Tokens::FnCall(i.to_string()));
                        found = true;
                        break;
                    }
                }
                if !found {
                    undefined_fn_calls.push((fn_name, index as usize, line.to_string()));
                }
            }
        }
        index += 1;
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

    tl
}
