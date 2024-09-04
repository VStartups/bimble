pub fn p_print(code: &str) -> String {
    if code.starts_with("echoln(\"") && code.ends_with("\")") {
        let start = "echoln(\"".len();
        let end = code.len() - "\")".len();
        return code[start..end].to_string();
    } else {
        eprintln!(
            "Error: The provided code '{}' does not match the expected format 'echoln(\"<text>\")'.",
            code
        );
        std::process::exit(1);
    }
}
