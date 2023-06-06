mod parser;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'f')]
    dotenv_files: Vec<String>,
    remain_args: Vec<String>,
}
fn main() {
    let args = Args::parse();
    for dotenv_file in &args.dotenv_files {
        if let Err(err) = dotenvy::from_filename(dotenv_file.clone()) {
            eprintln!("cannot load environment file `{dotenv_file}`: {err}");
            std::process::exit(1);
        }
    }

    if args.remain_args.len() == 0 {
        for (key, value) in dotenvy::vars() {
            println!("{key}={value}");
        }
        return;
    }
}
