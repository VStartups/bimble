use std::fmt;

#[derive(Debug, Clone)]
pub enum Tokens {
    Variable(String, Var, String), /* (varname : String , var : Var , usename : String) */
    Print(String),
    Takein(String),
    Func(String, TokenList),
    FnCall(String), /* Expand for args and more */
}

#[derive(Debug, Clone)]
pub struct TokenList(Vec<Tokens>);

impl Default for TokenList {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenList {
    pub fn new() -> Self {
        TokenList(Vec::new())
    }

    pub fn push(&mut self, token: Tokens) {
        self.0.push(token);
    }

    pub fn get(&self) -> &[Tokens] {
        &self.0
    }
    pub fn join_mut(&mut self, other: TokenList) {
        self.0.extend_from_slice(&other.0);
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::FnCall(nm) => {
                write!(f, "Function Call:\n╠───── Name => {}", nm)
            }
            Tokens::Variable(name, var, usename) => {
                write!(
                    f,
                    "Variable:\n╠───── Name => {}\n╠───── Type+Value => {},\n╠───── UseName => {}",
                    name, var, usename
                )
            }
            Tokens::Print(txt) => {
                write!(f, "Print:\n╠───── Text => {}", txt)
            }
            Tokens::Takein(vnm) => {
                write!(f, "Input:\n╠───── Variable Name => {}", vnm)
            }
            Tokens::Func(name, code) => {
                let code_str = code
                    .get()
                    .iter()
                    .map(|token| token.to_string()) // Convert each Tokens to String
                    .collect::<Vec<String>>() // Collect as Vec<String>
                    .join("    \n"); // Join with the desired separator

                write!(
                    f,
                    "Function:\n╠───── Name :  {}\n╠───── Codes : \n   {}",
                    name, code_str
                )
            }
        }
    }
}

impl fmt::Display for TokenList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for token in iter {
                write!(f, "\n\n{}", token)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Var {
    STR(String),
    INT(i64),
    F(f64),
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Var::STR(s) => write!(f, "String({})", s),
            Var::INT(i) => write!(f, "Integer({})", i),
            Var::F(flt) => write!(f, "Float({})", flt),
        }
    }
}
