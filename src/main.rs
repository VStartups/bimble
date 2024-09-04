pub mod gens;
pub mod gent;
pub mod tokens;
pub mod bcompile;

use std::{
    env, fmt,
    fs::File,
    io::{self, Read},
};

use bcompile::bc;


#[derive(Debug)]
enum AppError {
    FileNotFound(String),
    ReadError(String, io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileNotFound(path) => write!(f, "Error: File not found - '{}'", path),
            AppError::ReadError(path, err) => {
                write!(f, "Error: Unable to read file '{}'. Reason: {}", path, err)
            }
        }
    }
}

impl std::error::Error for AppError {}

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(AppError::FileNotFound("No file paths provided".to_string()));
    }
    //let scopes : Vec<Scope> = Vec::new();
    for path in &args[1..] {
        let path = path.trim();
        let mut file = File::open(path).map_err(|_| AppError::FileNotFound(path.to_string()))?;
        let mut code = String::new();
        file.read_to_string(&mut code)
            .map_err(|err| AppError::ReadError(path.to_string(), err))?;
        let tokens = gent::gen_token(&code);
        println!("{}", tokens);
        bc(tokens);
    }

    Ok(())
}
