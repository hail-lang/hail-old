pub mod ast;
pub mod scanner;

use clap::{Parser, Subcommand};
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[derive(Clone, Debug, Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "builds a source file")]
    Build {
        #[clap(help = "the hail source file to compile")]
        input: String,
    }
}

/// Arguments for the command line.
#[derive(Clone, Debug, Parser)]
#[clap(name = "hail")]
#[clap(about = "the hail compiler")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    match args.command {
        Command::Build { input } => {
            let file = match std::fs::read_to_string(input) {
                Ok(f) => f,
                Err(e) => {
                    println!("Unable to open input file.");
                    return Err(());
                },
            };
            let source = file.as_str();

            let start = std::time::Instant::now();
            let parser = grammar::RootStmntsParser::new();
            let ast = parser.parse(source, scanner::Asi::lex(source)).unwrap();
            let end = start.elapsed();

            dbg!(ast);
            println!("Parsed in {}ms", end.as_nanos() as f64 / 1_000_000f64);
        }
    }

    Ok(())

    // let input = std::fs::read_to_string("test.hl").unwrap();
    // let source = input.as_str();

    // let start = std::time::Instant::now();
    // let parser = grammar::RootStmntsParser::new();
    // let ast = parser.parse(source, scanner::Asi::lex(source)).unwrap();
    // let end = start.elapsed();

    // dbg!(ast);
    // println!("Parsed in {}ms", end.as_nanos() as f64 / 1_000_000f64);
}
