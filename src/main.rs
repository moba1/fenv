mod parser;
use clap::Parser;
use parser::ParseError;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "env command with dotenv")]
struct Args {
    /// dotenv file path. If you want to use multiple files, specify `-f file1 -f file2 ...`
    #[arg(short = 'f')]
    dotenv_files: Vec<String>,
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
        for (key, value) in dotenvy::vars() {
            println!("{key}={value}");
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
