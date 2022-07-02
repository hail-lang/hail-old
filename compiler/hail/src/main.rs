use hail_opt::{LyOpt, LyOptError};
use hail_diagnostic::{Diag, DiagEmitter, Label};
use hail_parser::grammar::DeclsParser;

fn main() {
    let opts = match LyOpt::parse() {
        Ok(opts) => opts,
        Err(e) => {
            let diag = match e {
                LyOptError::UnrecognizedOption(opt) => {
                    let label = Label::new(format!("I don't recognize the option '{}'.", opt));
                    let label2 = Label::new("Run `hail -h` for a list of options.");
                    Diag::error()
                        .with_code("E0005(cli)")
                        .with_labels(vec![label, label2])
                        .with_message("unrecognized option.")
                },
                LyOptError::MultipleInputs => {
                    let label = Label::new("I found multiple inputs in the command line arguments. \
                    I don't know which input to use.");
                    let label2 = Label::new("Usage: hail [OPTS] <INPUT>");
                    Diag::error()
                        .with_code("E0006(cli)")
                        .with_labels(vec![label, label2])
                        .with_message("multiple inputs.")
                },
                LyOptError::InvalidOptionArgs(opt) => {
                    let label = Label::new(format!("Invalid arguments for option '{}'", opt));
                    Diag::error()
                        .with_code("E0008(cli)")
                        .with_labels(vec![label])
                        .with_message("invalid arguments for option.")
                }
            };

            let mut emitter = DiagEmitter::new(false, "n/a", "");
            emitter.emit(diag).unwrap();
            return;
        }
    };

    let mut emitter = DiagEmitter::new(false, opts.input.as_str(), "");

    let source = match std::fs::read_to_string(opts.input.clone()) {
        Ok(source) => source,
        Err(_) => {
            let label = Label::new(format!("Unable to open input file! \
            Make sure that '{}' exists in the current working directory.", opts.input));
            let diag = Diag::error()
                .with_code("E0007(cli)")
                .with_labels(vec![label])
                .with_message("unable to open input file.");

            emitter.emit(diag).unwrap();
            return;
        }
    };

    emitter.set_source(&source);

    dbg!(DeclsParser::new().parse(&source).unwrap());
}