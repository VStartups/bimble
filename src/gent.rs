use crate::{
    gens::{print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};

pub fn gen_token(code: &str) -> TokenList {
    let mut tl = TokenList::new();

    for line in code.lines() {
        let line = line.trim();
        if line.starts_with("echoln(\"") && line.ends_with("\")") {
            let ptxt = p_print(line, &tl);
            tl.push(Tokens::Print(ptxt));
        } else if line.starts_with("may ") {
            let (name, var) = pvar(line);
            tl.push(Tokens::Variable(name, var));
        }
    }

    tl
}
