use crate::tokens::Var;

pub fn pvar(code: &str) -> (String, Var) {
    let code_trimmed = match code.strip_prefix("may ") {
        Some(code) => code.trim(),
        None => {
            eprintln!("Error: Prefix 'may ' not found in code '{}'", code);
            std::process::exit(1);
        }
    };

    let mut pts = code_trimmed.splitn(2, '=');
    let name = pts.next().expect("No name found").trim();
    let value = pts.next().expect("No value found").trim();

    let v = parse_value(value, code);

    (name.to_string(), v)
}

fn parse_value(value: &str, code: &str) -> Var {
    let trimmed_value = value.trim();
    if trimmed_value.starts_with('"') && trimmed_value.ends_with('"') {
        return Var::STR(trimmed_value[1..trimmed_value.len() - 1].to_string());
    }
    if let Ok(parsed) = trimmed_value.parse::<i128>() {
        return Var::INT(parsed);
    }
    if let Ok(parsed) = trimmed_value.parse::<f64>() {
        if trimmed_value.contains('.') {
            return Var::F(parsed);
        }
    }

    eprintln!(
        "Error: Invalid value for variable: '{}' || Code => {}",
        value, code
    );
    std::process::exit(1);
}
