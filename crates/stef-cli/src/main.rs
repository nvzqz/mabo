mod cli;

use std::{fs, path::PathBuf, process::ExitCode};

use miette::{Context, IntoDiagnostic, Result};
use stef_parser::Schema;

use self::cli::Cli;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Some(cmd) = cli.cmd {
        let result = match cmd {
            cli::Command::Init { path } => {
                println!("TODO: create basic setup at {path:?}");
                Ok(())
            }
            cli::Command::Check { files } => check(files),
            cli::Command::Format { files } => format(files),
        };

        return match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{e:?}");
                ExitCode::FAILURE
            }
        };
    }

    ExitCode::SUCCESS
}

fn check(files: Vec<PathBuf>) -> Result<()> {
    for file in files {
        let buf = fs::read_to_string(&file)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed reading {file:?}"))?;

        if let Err(e) = Schema::parse(&buf).wrap_err("Failed parsing schema file") {
            eprintln!("{e:?}");
        }
    }

    Ok(())
}

fn format(files: Vec<PathBuf>) -> Result<()> {
    for file in files {
        let buf = fs::read_to_string(&file).into_diagnostic()?;
        let schema = match Schema::parse(&buf).wrap_err("Failed parsing schema file") {
            Ok(schema) => schema,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };

        let formatted = schema.to_string();

        if buf != formatted {
            fs::write(file, &buf).into_diagnostic()?;
        }
    }

    Ok(())
}
