pub mod bcompile;
pub mod gencc;
pub mod genfc;
pub mod gens;
pub mod gent;
pub mod tokens;

use std::{
    env,
    fmt,
    fs::File,
    io::{self, Read},
};

use bcompile::{check_compiler, compile_code};

#[derive(Debug)]

enum AppError
{
    FileNotFound(String),
    ReadError(String, io::Error),
}

impl std::fmt::Display for AppError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {

        match self
        {
            AppError::FileNotFound(path) => write!(f, "Error: File not found - '{}'", path),
            AppError::ReadError(path, err) =>
            {

                write!(f, "Error: Unable to read file '{}'. Reason: {}", path, err)
            }
        }
    }
}

impl std::error::Error for AppError {}

fn main() -> Result<(), AppError>
{

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty()
    {

        return Err(AppError::FileNotFound("No file paths provided".to_string()));
    }

    for path in args
    {

        let path = path.trim();

        let mut file = File::open(path).map_err(|_| AppError::FileNotFound(path.to_string()))?;

        let mut code = String::new();

        file.read_to_string(&mut code).map_err(|err| AppError::ReadError(path.to_string(), err))?;

        let tokens = gent::gen_token(&code);

        // println!("Tokens:\n {}", tokens);

        match env::consts::OS
        {
            "windows" =>
            {

                check_compiler("clang");

                compile_code(tokens, "clang");
            }
            "macos" | "linux" =>
            {

                check_compiler("gcc");

                compile_code(tokens, "gcc");
            }
            _ => eprintln!("Unsupported operating system: {}", env::consts::OS),
        }
    }

    Ok(())
}
