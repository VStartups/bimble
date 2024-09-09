use crate::tokens::Var;

pub fn pvar(
    code: &str,
    vrs: &mut Vec<(String, i32)>,
) -> (String, Var, String)
{

    // let mut vrs : Vec<(String,i32)> /*  (og name : String , counter : i32) */ =
    // Vec::new();
    let code_trimmed =
        code.strip_prefix("may ").map(str::trim).expect("Prefix 'may ' not found in code");

    let mut parts = code_trimmed.splitn(2, '=');

    let name = parts.next().expect("No name found").trim().to_string();

    let mut usename = String::new();

    for i in &mut *vrs
    {

        if i.0 == name
        {

            i.1 += 1;

            usename = format!("{}_{}", name, i.1);

            // println!("var alr there new name : {}", usename);
            i.1 += 1;

            break;
        }
    }

    if usename.is_empty()
    {

        usename = name.clone();

        vrs.push((usename.clone(), 0));
    }

    let value = parts.next().expect("No value found").trim();

    let var_value = parse_value(value);

    (name, var_value, usename)
}

fn parse_value(value: &str) -> Var
{

    let trimmed_value = value.trim();

    if trimmed_value.starts_with('"') && trimmed_value.ends_with('"')
    {

        return Var::STR(trimmed_value[1..trimmed_value.len() - 1].to_string());
    }

    if let Ok(parsed) = trimmed_value.parse::<i64>()
    {

        return Var::INT(parsed);
    }

    if let Ok(parsed) = trimmed_value.parse::<f64>()
    {

        if trimmed_value.contains('.')
        {

            return Var::F(parsed);
        }
    }

    panic!("Error: Invalid value for variable: '{}' || Code => {}", value, trimmed_value)
}
