use crate::tokens::Var;

pub fn pvar(code: &str) -> (String, Var) {
    let code_trimmed = code
        .strip_prefix("may ")
        .map(str::trim)
        .expect("Prefix 'may ' not found in code");

    let mut parts = code_trimmed.splitn(2, '=');
    let name = parts.next().expect("No name found").trim().to_string();
    let value = parts.next().expect("No value found").trim();

    let var_value = parse_value(value);

    (name, var_value)
}

fn parse_value(value: &str) -> Var {
    let trimmed_value = value.trim();
    if trimmed_value.starts_with('"') && trimmed_value.ends_with('"') {
        return Var::STR(trimmed_value[1..trimmed_value.len() - 1].to_string());
    }
    if let Ok(parsed) = trimmed_value.parse::<i64>() {
        return Var::INT(parsed);
    }
    if let Ok(parsed) = trimmed_value.parse::<f64>() {
        if trimmed_value.contains('.') {
            return Var::F(parsed);
        }
    }
    panic!(
        "Error: Invalid value for variable: '{}' || Code => {}",
        value, trimmed_value
    )
}
