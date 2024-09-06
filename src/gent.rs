use std::process::exit;

use crate::{
    gens::{pin::pin, print::p_print, var::pvar},
    tokens::{TokenList, Tokens},
};

pub fn gen_token(code: &str) -> TokenList {
    let mut tl = TokenList::new();
    let mut index = 1;
    for line in code.lines() {
        let line = line.trim();
        if line.starts_with("echoln(\"") && line.ends_with("\")") {
            let ptxt = p_print(line, &tl);
            tl.push(Tokens::Print(ptxt));
        } else if line.starts_with("may ") {
            let (name, var) = pvar(line);
            tl.push(Tokens::Variable(name, var));
        }
        else if line.starts_with("takein(") && line.ends_with(")"){
            let g = pin(&line , &tl);
            if !g.0{
                eprintln!("Error : Unable to find variable '{}' => Code[{}] : {}",g.1,index,line);
                exit(1);
            }
            tl.push(Tokens::Takein(g.1));
            
        }
        index += 1;
    }

    tl
}
