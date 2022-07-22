use std::path::PathBuf;

use clap::{Parser, Subcommand};
use hailc::Compiler;
use hailc::diag::builder::UnwrapOrEmit;
use hailc::diag::codespan::CodespanDriver;

#[derive(Clone, Debug, Parser)]
#[clap(name = "hail")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "A compiler for the Hail programming language", long_about = None)]
struct Hail {
    #[clap(subcommand)]
    subcommand: HailSubcommand,
}

#[derive(Clone, Debug, Subcommand)]
enum HailSubcommand {
    #[clap(arg_required_else_help = true)]
    Build {
        #[clap(required = true, value_parser)]
        #[clap(help = "The input file to compile")]
        input: PathBuf,

        #[clap(value_parser)]
        #[clap(help = "Sets the output path")]
        #[clap(short, long, value_name = "OUTPUT PATH")]
        #[clap(alias = "out")]
        output: Option<PathBuf>,

        #[clap(value_parser)]
        #[clap(help = "Sets the name of the module I'm compiling")]
        #[clap(short, long, value_name = "MODULE NAME")]
        module: Option<String>,

        #[clap(value_parser)]
        #[clap(help = "Adds a directory where Hail modules can be found")]
        #[clap(short = 'M', long, value_name = "MODULE PATH")]
        mpath: Vec<String>,
    },
}

fn main() {
    let args = Hail::parse();

    match args.subcommand {
        HailSubcommand::Build { input, .. } => {
            let source = match std::fs::read_to_string(&input) {
                Ok(str) => str,
                Err(_) => {
                    println!("cannot open input file, for whatever reason.");
                    std::process::exit(1);
                },
            };
            let input = input.to_string_lossy();

            let mut compiler = Compiler::for_file(&input, &source, CodespanDriver::default());
            let files = compiler.files();
            let toks = compiler.lex().unwrap_or_emit(&files);
            let _ast = compiler.parse(toks).unwrap_or_emit(&files);
            //dbg!(res);
        },
    }
}
