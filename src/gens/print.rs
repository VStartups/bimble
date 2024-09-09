use crate::tokens::{TokenList, Tokens};
use std::process::exit;

pub fn p_print(
    code: &str,
    tl: &TokenList,
) -> String
{

    if !code.starts_with("echoln(\"") || !code.ends_with("\")")
    {

        eprintln!("Error: Invalid format. Code: '{}'", code);

        exit(1);
    }

    let txt = &code["echoln(\"".len()..code.len() - 2]; // Extract the actual text inside echoln
    let mut result = String::with_capacity(txt.len()); // Preallocate capacity
    let mut variable_name_caught = String::new();

    let mut inside_var = false;

    //println!("Processing text: '{}'", txt);

    for c in txt.chars()
    {

        match c
        {
            '$' =>
            {

                if inside_var
                {

                    eprintln!("Error: Unexpected '$' inside variable mode. Code: '{}'", code);

                    exit(1);
                }

                if !variable_name_caught.is_empty()
                {

                    //println!("Flushing literal text: '{}'", variable_name_caught);

                    result.push_str(&variable_name_caught); // Flush any literal text
                    variable_name_caught.clear();
                }

                result.push(c); // Add the dollar sign
                inside_var = true; // Start processing a variable
                //println!("Entering variable mode with '$'");
            }
            ' ' | ';' | ',' | '.' | ':' =>
            {

                if inside_var
                {

                    //println!("Processing variable: '{}'", variable_name_caught);

                    process_variable(&variable_name_caught, tl, &mut result, code);

                    variable_name_caught.clear();

                    inside_var = false;
                }

                result.push(c);

                //println!("Appending special character or space: '{}'", c);
            }
            _ if inside_var =>
            {

                variable_name_caught.push(c); // Continue collecting the variable name
               // println!("Collecting variable character: '{}'", c);
            }
            _ =>
            {

                if inside_var
                {

                    // println!(
                    //     "Processing variable before appending character: '{}'",
                    //     variable_name_caught
                    // );

                    process_variable(&variable_name_caught, tl, &mut result, code);

                    variable_name_caught.clear();

                    inside_var = false;
                }

                result.push(c); // Collect literal text
                //println!("Appending literal character: '{}'", c);
            }
        }
    }

    if inside_var
    {

        //println!("Processing remaining variable: '{}'", variable_name_caught);

        process_variable(&variable_name_caught, tl, &mut result, code);
    }

    result
}

fn process_variable(
    variable_name_caught: &str,
    tl: &TokenList,
    result: &mut String,
    code: &str,
)
{

    let variable_name = variable_name_caught; // Remove the '$' sign
    //println!("Processing variable name: '{}'", variable_name);

    if tl.get().iter().any(|token| {
        if let Tokens::Variable(name, ..) = token
        {

            *name == variable_name
        }
        else
        {

            false
        }
    })
    {

        result.push_str(variable_name); // Add the variable name to the result
        //println!("Variable '{}' found and added to result", variable_name);
    }
    else
    {

        eprintln!("Variable '{}' not found. Code: \n{}", variable_name, code);

        exit(1);
    }
}
