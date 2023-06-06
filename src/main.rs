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
    println!("{:?}", args);
}
