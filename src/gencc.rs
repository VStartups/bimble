use crate::tokens::{TokenList, Tokens, Var};
use std::collections::HashMap;

pub fn gen_cc(tokens: TokenList) -> String
{

    let mut vars = Vec::new();

    let mut var_names_count = HashMap::new();

    let includes = "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n".to_string();

    let mut func_defs = String::new();

    let mut main_code = String::new();

    let mut updated_tokens = Vec::new();

    for token in tokens.get()
    {

        match token
        {
            Tokens::Variable(name, ty, use_name) =>
            {

                let unique_use_name = get_unique_use_name(&use_name, &mut var_names_count);

                updated_tokens.push(Tokens::Variable(
                    name.clone(),
                    ty.clone(),
                    unique_use_name.clone(),
                ));

                vars.push((name.clone(), ty.clone(), unique_use_name.clone()));

                let declaration = match ty
                {
                    Var::STR(s) =>
                    {

                        format!(
                            "char {}[{}] = \"{}\";\n",
                            unique_use_name,
                            unique_use_name.len() + 100,
                            s
                        )
                    }
                    Var::F(f) => format!("double {} = {};\n", unique_use_name, f),
                    Var::INT(i) => format!("long long {} = {};\n", unique_use_name, i),
                };

                main_code.push_str(&declaration);
            }
            Tokens::Func(name, code) =>
            {

                let nested_code = crate::genfc::gen_fc(code.clone(), &vars, &mut var_names_count);

                func_defs.push_str(&format!("void {}(){{\n{}\n}}\n", name, nested_code));
            }
            Tokens::FnCall(name) =>
            {

                main_code.push_str(&format!("{}();\n", name));
            }
            Tokens::Print(text) =>
            {

                let print_code = generate_print_code(&text, &vars);

                main_code.push_str(&print_code);
            }
            Tokens::Takein(name) =>
            {

                let input_code = format!(
                    "fgets({}, sizeof({}), stdin);\n{}[strcspn({}, \"\\n\")] = 0;\n",
                    name, name, name, name
                );

                main_code.push_str(&input_code);
            }
        }
    }

    let code = format!("{}\n{}\nint main(){{\n{}\nreturn 0;\n}}", includes, func_defs, main_code);

    code
}

fn get_unique_use_name(
    base_name: &str,
    names_count: &mut HashMap<String, usize>,
) -> String
{

    let count = names_count.entry(base_name.to_string()).or_insert(0);

    *count += 1;

    if *count == 1
    {

        base_name.to_string()
    }
    else
    {

        format!("{}_{}", base_name, *count - 1)
    }
}

const SPECIAL_SYMBOLS: [char; 33] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '{', '}', '[', ']', ':',
    ';', '"', '\'', '<', '>', ',', '.', '/', '?', '|', '\\', '~', '`', ' ',
];

fn generate_print_code(
    text: &str,
    vars: &[(String, Var, String)],
) -> String
{

    let mut output = String::new();

    output.push_str("printf(\"");

    let mut inv = false;

    let mut vrwrd = String::new();

    let mut vrs = Vec::new();

    for i in text.chars()
    {

        if i == '$' && !inv
        {

            inv = true;
        }
        else if inv && !SPECIAL_SYMBOLS.contains(&i)
        {

            vrwrd.push(i);
        }
        else if inv && SPECIAL_SYMBOLS.contains(&i)
        {

            inv = false;

            for v in vars
            {

                if v.0 == vrwrd
                {

                    vrs.push(v.2.clone());

                    match v.1
                    {
                        Var::INT(_) =>
                        {

                            output.push_str(format!("{}%lld", i).as_str());
                        }
                        Var::STR(_) =>
                        {

                            output.push_str(format!("{}%s", i).as_str());
                        }
                        Var::F(_) =>
                        {

                            output.push_str(format!("{}%f", i).as_str());
                        }
                    }
                }
            }
        }
        else
        {

            if !inv
            {

                output.push(i);
            }
        }
    }

    for v in vars
    {

        if v.0 == vrwrd
        {

            vrs.push(v.2.clone());

            match v.1
            {
                Var::INT(_) =>
                {

                    output.push_str(format!("%lld").as_str());
                }
                Var::STR(_) =>
                {

                    output.push_str(format!("%s").as_str());
                }
                Var::F(_) =>
                {

                    output.push_str(format!("%f").as_str());
                }
            }
        }
    }

    if !vrs.is_empty()
    {

        output.push_str(format!("\\n\",{});", vrs.join(",")).as_str());
    }
    else
    {

        output.push_str(format!("\\n\");").as_str());
    }

    output
}
