use crate::tokens::{TokenList, Tokens};

pub fn pin(code: &str, tl: &TokenList) -> (bool, String) {
    let nm = &code[7..].trim();
    let nm = nm.trim_end_matches(")");
    println!("Debug => NM : {}", nm);
    for i in tl.get() {
        match i {
            Tokens::Variable(nmm, _, _) => {
                if nmm == nm {
                    return (true, nmm.to_string());
                }
            }
            _ => continue,
        }
    }
    (false, nm.to_string())
}
