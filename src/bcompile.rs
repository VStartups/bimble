use crate::tokens::TokenList;
use indicatif::ProgressBar;
use std::{
    fs::{self, File},
    io::Write,
    process::{exit, Command},
    time::Duration,
};

pub fn check_compiler(compiler: &str)
{

    let output = Command::new(compiler).arg("--version").output().unwrap_or_else(|_| {

        eprintln!("{} is not installed or not found in the system's PATH.", compiler);

        exit(1);
    });

    if !output.status.success()
    {

        eprintln!("{} is not installed or not found in the system's PATH.", compiler);

        exit(1);
    }
}

pub fn show_progress_bar()
{

    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(100));

    pb.set_message("Compiling...");

    std::thread::sleep(Duration::from_secs(2));

    pb.finish_with_message("Compilation finished.");
}

pub fn compile_code(
    tokens: TokenList,
    compiler: &str,
)
{

    let code = crate::gencc::gen_cc(tokens);

    let mut file = File::create("t.c").unwrap_or_else(|_| {

        eprintln!("Unable to create 't.c'. Compilation failed.");

        exit(1);
    });

    if let Err(e) = file.write_all(code.as_bytes())
    {

        eprintln!("Failed to write C code to 't.c': {}", e);

        exit(1);
    }

    compile_with(compiler);
}

pub fn compile_with(compiler: &str)
{

    let pb = ProgressBar::new_spinner();

    pb.set_message("Compiling...");

    let compile_output = Command::new(compiler)
        .arg("-c")
        .arg("t.c")
        .arg("-o")
        .arg("t.o")
        .arg("-O3")
        .output()
        .unwrap_or_else(|_| {

            eprintln!("Failed to execute {}.", compiler);

            exit(1);
        });

    pb.enable_steady_tick(Duration::from_millis(50));

    while !compile_output.status.success()
    {

        std::thread::sleep(Duration::from_millis(100));
    }

    pb.finish_with_message("Compilation complete.");

    if !compile_output.status.success()
    {

        eprintln!(
            "Compilation failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&compile_output.stderr)
        );

        exit(1);
    }

    let link_output =
        Command::new(compiler).arg("t.o").arg("-o").arg("t").output().unwrap_or_else(|_| {

            eprintln!("Failed to execute {} for linking.", compiler);

            exit(1);
        });

    pb.set_message("Linking...");

    pb.enable_steady_tick(Duration::from_millis(50));

    while !link_output.status.success()
    {

        std::thread::sleep(Duration::from_millis(100));
    }

    if !link_output.status.success()
    {

        eprintln!(
            "Linking failed with {}: {}",
            compiler,
            String::from_utf8_lossy(&link_output.stderr)
        );

        exit(1);
    }

    pb.finish_with_message("Finished building");

    fs::remove_file("t.o").unwrap_or_else(|_| {

        eprintln!("Failed to delete temporary file 't.o'.");

        exit(1);
    });
}
