//! Options for the `hail` command line.

/// The options usable on the `hail` command line.
#[derive(Clone, Debug, PartialEq)]
pub struct LyOpt {
    /// The input to parse.
    pub input: String,

    /// Whether or not any colors are allowed to be used on the command line.
    pub no_color: bool,
}

/// Errors while parsing arguments.
pub enum LyOptError {
    /// Only one input is allowed.  Multiple were found.
    MultipleInputs,

    /// An unrecognized option was found.
    UnrecognizedOption(String),
}

impl LyOpt {
    /// Parses the command line options into an `LyOpt` struct.
    pub fn parse() -> Result<Self, LyOptError> {
        let mut args = std::env::args();
        args.next();

        let mut input = None;
        let mut no_color = false;

        for arg in args {
            if &arg[..1] == "-" {
                match arg.as_str() {
                    "-no_color" | "-nocolor" => no_color = true,
                    "-v" | "-version" => {
                        println!("hail v{}", env!("CARGO_PKG_VERSION"));
                        std::process::exit(0);
                    },
                    "-h" | "-help" => {
                        println!("hail v{}", env!("CARGO_PKG_VERSION"));
                        println!("Usage: hail [OPTIONS] <INPUT>");
                        println!("");
                        println!("Options:");
                        println!("     -h, -help             Displays this message.");
                        println!("     -no-color             Disables command line colors.");
                        println!("     -v, -version          Displays the version of `hail` that's running.");
                        std::process::exit(0);
                    },
                    _ => return Err(LyOptError::UnrecognizedOption(arg)),
                }

                continue;
            }

            if let Some(_) = input {
                return Err(LyOptError::MultipleInputs);
            }

            input = Some(arg);
        }

        match input {
            Some(input) => Ok(Self {
                input,
                no_color,
            }),
            None => {
                println!("hail v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
        }
    }
}