mod parser;
use clap::Parser;
use is_terminal::IsTerminal;
use parser::ParseError;
use std::process::exit;
use yansi::Paint;

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
enum ColorMode {
    Never,
    Auto,
    Always,
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "env command with dotenv")]
struct Args {
    /// dotenv file path
    #[arg(short = 'f')]
    dotenv_files: Vec<String>,
    /// color mode
    #[arg(long = "color", default_value = "auto", verbatim_doc_comment)]
    color_mode: ColorMode,
    /// environment set and comand arguments [format: [NAME=VALUE]... [COMMAND [ARG]...]]
    #[arg(value_name = "ARGUMENTS", verbatim_doc_comment)]
    remain_args: Vec<String>,
}
fn main() {
    let args = Args::parse();
    for dotenv_file in &args.dotenv_files {
        if let Err(err) = dotenvy::from_filename(dotenv_file.clone()) {
            eprintln!("cannot load environment file `{dotenv_file}`: {err}");
            exit(1);
        }
    }
    let color_mode = args.color_mode;

    let mut args = args.remain_args.into_iter();
    let mut program: Option<String> = None;
    loop {
        match args.next() {
            None => break,
            Some(arg) => match parser::parse(&arg) {
                Ok(env_var) => std::env::set_var(env_var.key, env_var.value),
                Err(ParseError::NotEnvVar) => program = Some(arg),
            },
        }
    }

    if program.is_none() {
        let is_tty = std::io::stdout().is_terminal();
        for (key, value) in dotenvy::vars() {
            if color_mode == ColorMode::Always || (color_mode == ColorMode::Auto && is_tty) {
                println!("{}={}", Paint::green(key), Paint::blue(value));
            } else {
                println!("{key}={value}")
            }
        }
        return;
    }

    let command = std::process::Command::new(program.clone().unwrap())
        .args(args)
        .envs(dotenvy::vars())
        .status();
    match command {
        Err(err) => {
            eprintln!("cannot spawn program `{}`: {err}", program.unwrap());
            exit(2);
        }
        Ok(exit_status) => exit(exit_status.code().unwrap_or(1)),
    }
}
