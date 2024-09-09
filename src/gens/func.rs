use std::process::exit;

use crate::{
    gens::{pin::pin, print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};

pub fn p_fn(
    code: &str,
    vrs: &mut Vec<(String, i32)>,
    index: &mut i32,
) -> (String, TokenList, Vec<String>, Vec<(String, i32)>)
{

    let lines: Vec<&str> = code.lines().collect();

    let mut flist = Vec::new();

    let mut tl = TokenList::new();

    let mut function_body = TokenList::new();

    let mut infn = false;

    let mut fn_name = String::new();

    for ln in &lines
    {

        let ln = ln.trim();

        if ln.starts_with("ON ") && ln.ends_with("{")
        {

            fn_name = extract_fn_name(ln);

            if flist.contains(&fn_name)
            {

                eprintln!("Error: Function '{}' is already defined.", fn_name);

                exit(1);
            }

            flist.push(fn_name.clone());

            infn = true;
        }
        else if infn
        {

            if ln == "}"
            {

                infn = false;

                tl.push(Tokens::Func(fn_name.clone(), function_body.clone()));
            }
            else
            {

                process_function_body(ln, &mut function_body, vrs);
            }
        }
        else
        {

            process_line(ln, &mut tl, vrs, index);
        }

        *index += 1;
    }

    (fn_name, tl, flist, vrs.to_vec())
}

fn process_function_body(
    ln: &str,
    function_body: &mut TokenList,
    vrs: &mut Vec<(String, i32)>,
)
{

    if ln.starts_with("echoln(\"") && ln.ends_with("\")")
    {

        let ptxt = p_print(ln, function_body);

        function_body.push(Tokens::Print(ptxt));
    }
    else if ln.starts_with("may ")
    {

        let (name, var, usename) = pvar(ln, vrs);

        function_body.push(Tokens::Variable(name.clone(), var.clone(), usename.clone()));
    }
    else if ln.starts_with("takein(") && ln.ends_with(")")
    {

        let g = pin(ln, function_body);

        if g.0
        {

            function_body.push(Tokens::Takein(g.1.clone()));
        }
    }
}

fn process_line(
    ln: &str,
    tl: &mut TokenList,
    vrs: &mut Vec<(String, i32)>,
    _index: &mut i32,
)
{

    if ln.starts_with("may ")
    {

        let (name, var, usename) = pvar(ln, vrs);

        tl.push(Tokens::Variable(name.clone(), var.clone(), usename.clone()));
    }
}

fn extract_fn_name(ln: &str) -> String
{

    ln.split_whitespace().nth(1).unwrap_or_default().to_string()
}
