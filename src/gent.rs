use crate::{
    gens::{func::p_fn, pin::pin, print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};
use std::process::exit;

pub fn gen_token(code: &str) -> TokenList
{

    let mut tl = TokenList::new();

    let mut index = 1;

    let mut vrs: Vec<(String, i32)> = Vec::new();

    let mut flist: Vec<String> = Vec::new(); // List of defined functions
    let mut function_body = String::new(); // Accumulate function body
    let mut infn = false;

    let mut function_calls: Vec<(String, usize, String)> = Vec::new(); // Store function calls for processing later

    for line in code.lines()
    {

        let line = line.trim();

        if line.is_empty()
        {

            index += 1;

            continue;
        }

        // Handle echoln
        if line.starts_with("echoln(\"") && line.ends_with("\")") && !infn
        {

            let ptxt = p_print(line, &tl);

            tl.push(Tokens::Print(ptxt.clone()));
        }
        // Handle variable definition (may)
        else if line.starts_with("may ") && !infn
        {

            let (name, var, usename) = pvar(line, &mut vrs);

            tl.push(Tokens::Variable(name.clone(), var.clone(), usename.clone()));
        }
        // Handle takein
        else if line.starts_with("takein(") && line.ends_with(")") && !infn
        {

            let g = pin(line, &tl);

            if !g.0
            {

                eprintln!(
                    "Error: Undefined variable '{}' used in takein statement. Line {}: '{}'",
                    g.1, index, line
                );

                exit(1);
            }

            tl.push(Tokens::Takein(g.1.clone()));
        }
        // Handle function definition
        else if line.starts_with("ON ") && line.ends_with("{")
        {

            infn = true;

            function_body.push_str(format!("\n{}", line).as_str());
        }
        // If inside a function body
        else if infn
        {

            function_body.push_str(format!("\n{}", line).as_str());

            if line.trim() == "}"
            {

                infn = false;

                let fpr = p_fn(&function_body, &mut vrs, &mut index);

                flist.extend(fpr.2); // Extend the function list
                tl.join_mut(fpr.1); // Merge tokens from the function
                function_body.clear(); // Clear after processing
            }
        }
        // Accumulate function calls
        else
        {

            let fn_name = line.split('(').next().unwrap_or("").to_string();

            function_calls.push((fn_name, index.try_into().unwrap(), line.to_string()));
        }

        index += 1;
    }

    // Process function calls
    let mut notfound = false;

    for (fn_name, line_index, full_line) in function_calls
    {

        if !flist.contains(&fn_name)
        {

            eprintln!(
                "Error: Undefined function call '{}' at line {}: '{}'",
                fn_name, line_index, full_line
            );

            notfound = true;
        }
        else
        {

            tl.push(Tokens::FnCall(fn_name));
        }
    }

    if notfound
    {

        exit(1);
    }

    // println!("[DEBUG] Token generation complete.");
    tl
}
