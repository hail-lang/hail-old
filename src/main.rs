// TODO: array types & array values

pub mod hir_lower;

use clap::{Parser, Subcommand};
use target_lexicon::Triple;

#[derive(Clone, Debug, Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    #[clap(about = "builds a source file")]
    Build {
        /// The input file to compile.
        #[clap(help = "the hail source file to compile")]
        input: String,

        /// The flags to compile with.
        #[clap(short = 'f', long = "flag")]
        #[clap(help = "registers a flag for conditional compilation")]
        flags: Vec<String>,

        /// Directories that hail should search for modules.
        #[clap(short = 'M', long = "lib")]
        #[clap(help = "registers a path to search for modules")]
        libs: Vec<String>,

        #[clap(long = "bench")]
        #[clap(help = "display how long hail spends in each pass")]
        bench: bool,
    },
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

// TODO: load flags for different targets

fn main() -> Result<(), ()> {
    let args = Args::parse();

    match args.command {
        Command::Build { input, flags, libs, bench } => {
            let file = match std::fs::read_to_string(input) {
                Ok(f) => f,
                Err(e) => {
                    println!("Unable to open input file.");
                    return Err(());
                },
            };
            let source = file.as_str();

            let ast = {
                let start = std::time::Instant::now();
                let parser = hail_parser::grammar::RootStmntsParser::new();
                let ast = parser.parse(source, hail_parser::scanner::Asi::lex(source)).unwrap();
                let end = start.elapsed();
    
                //dbg!(ast);
                if bench {
                    println!("Parsed in {}ms", end.as_nanos() as f64 / 1_000_000f64);
                }
                
                ast
            };

            {
                let mut ctx = hir_lower::HirLowerContext {
                    flags,
                    libs,
                };
                let unit = hir_lower::HirLowerUnit{
                    dir: "./".into(),
                    ast,
                };

                let start = std::time::Instant::now();
                hir_lower::hir_lower(&mut ctx, &unit);
                let end = start.elapsed();

                if bench {
                    println!("Lowered to HIR in {}ms", end.as_nanos() as f64 / 1_000_000f64);
                }
            }
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
