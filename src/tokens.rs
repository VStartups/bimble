use std::fmt;

#[derive(Debug)]
pub enum Tokens {
    Variable(String, Var),
    Print(String),
    Takein(String),
}

pub struct TokenList(Vec<Tokens>);

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
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::Variable(name, var) => {
                write!(
                    f,
                    "Variable:\n╠───── Name => {}\n╠───── Type+Value => {}",
                    name, var
                )
            }
            Tokens::Print(txt) => {
                write!(f, "Print:\n╠───── Text => {}", txt)
            }
            Tokens::Takein(vnm) => {
                write!(f, "Input:\n╠───── Variable Name => {}", vnm)
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

#[derive(Debug)]
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
