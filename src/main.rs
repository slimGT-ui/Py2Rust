use py2rust::{InputError, load_python_file};
use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::PathBuf;
use std::process;

fn main() {
    let code = match run(env::args_os().skip(1)) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Try 'py2rust --help' for usage.");
            error.exit_code()
        }
    };
    process::exit(code);
}

fn run(args: impl Iterator<Item = OsString>) -> Result<(), CliError> {
    let mut args = args;
    let Some(first) = args.next() else {
        return Err(CliError::Usage("missing input file".to_owned()));
    };

    if first == OsStr::new("--help") || first == OsStr::new("-h") {
        print_help();
        return Ok(());
    }
    if first == OsStr::new("--version") || first == OsStr::new("-V") {
        println!("py2rust {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if first.to_string_lossy().starts_with('-') {
        return Err(CliError::Usage(format!("unknown option: {}", first.to_string_lossy())));
    }
    if args.next().is_some() {
        return Err(CliError::Usage("expected exactly one input file".to_owned()));
    }

    let input = load_python_file(PathBuf::from(first)).map_err(CliError::Input)?;
    println!("Loaded Python source: {} ({} bytes)", input.path.display(), input.source.len());
    println!("Translation is not implemented yet.");
    Ok(())
}

fn print_help() {
    println!(
        "py2rust {}\nA Python-to-Rust transpiler.\n\nUSAGE:\n    py2rust <INPUT.py>\n    py2rust --help\n    py2rust --version\n\nOPTIONS:\n    -h, --help       Print help information\n    -V, --version    Print version information\n\nThis build validates and loads one input file; translation is not implemented yet.",
        env!("CARGO_PKG_VERSION")
    );
}

#[derive(Debug)]
enum CliError {
    Usage(String),
    Input(InputError),
}

impl CliError {
    fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 2,
            Self::Input(_) => 1,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => f.write_str(message),
            Self::Input(error) => error.fmt(f),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Usage(_) => None,
            Self::Input(error) => Some(error),
        }
    }
}
