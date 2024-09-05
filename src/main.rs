pub mod bcompile;
pub mod gens;
pub mod gent;
pub mod tokens;

use std::{
    env, fmt,
    fs::File,
    io::{self, Read},
};

use bcompile::{bc_clang, bc_gcc};

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
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err(AppError::FileNotFound("No file paths provided".to_string()));
    }

    for path in args {
        let path = path.trim();
        let mut file = File::open(&path).map_err(|_| AppError::FileNotFound(path.to_string()))?;
        let mut code = String::new();
        file.read_to_string(&mut code)
            .map_err(|err| AppError::ReadError(path.to_string(), err))?;
        let tokens = gent::gen_token(&code);
        let os = env::consts::OS;

        //let tokens = TokenList::new(); // Placeholder for actual tokens
        match os {
            "windows" => bc_clang(tokens),
            "macos" | "linux" => bc_gcc(tokens),
            _ => eprintln!("Unsupported operating system: {}", os),
        }
        //bc_gcc(tokens);
    }

    Ok(())
}
