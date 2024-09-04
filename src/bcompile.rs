use std::{
    fs::File, io::Write, process::{exit, Command}
};

use crate::tokens::{TokenList, Tokens, Var};

pub fn bc(tokens: TokenList) {
    checkgcc();
    let mut cfile: File;
    match File::create("t.cpp") {
        Ok(cf) => {
            cfile = cf;
        }
        Err(_) => {
            eprintln!("Unable to compile Bimble.....");
            exit(1);
        }
    }
    let mut cc = String::new();
    cc.push_str("#include <iostream>\n#include <string>\nusing namespace std;\nint main(){");
    for i in tokens.get() {
        match i {
            Tokens::Print(txt) => {
                cc.push_str(format!("\ncout << \"{}\" << endl;", txt).as_str());
            }
            Tokens::Variable(nm, ty) => match ty {
                Var::STR(txt) => {
                    cc.push_str(format!("\nstd::string {} = \"{}\";", nm, txt).as_str());
                }
                Var::F(f) => {
                    cc.push_str(format!("\ndouble {} = {};", nm, f).as_str());
                }
                Var::INT(i) => {
                    cc.push_str(format!("\nlong long {} = {}LL;", nm, i).as_str());
                }
            },
        }
    }
    cc.push_str("\nreturn 0;\n}");

    cfile.write_all(cc.as_bytes()).unwrap();
    
    let cmd = Command::new("g++").arg("t.cpp").arg("-o").arg("t.out").output().unwrap();
    
    if cmd.status.success() {
        println!("Compilation succeeded.");
    } else {
        eprintln!("Compilation failed: {}", String::from_utf8_lossy(&cmd.stderr));
    }
}

fn checkgcc() {
    let out = Command::new("g++").arg("--version").output();

    match out {
        Ok(output) => {
            if output.status.success() {
                // g++ is available, continue
            } else {
                eprintln!("g++ is installed, but there was an issue running the command.");
                exit(1);
            }
        }
        Err(_) => {
            eprintln!("g++ is not installed or not found in the system's PATH.");
            exit(1);
        }
    }
}
