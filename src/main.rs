use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod scanner;

lalrpop_mod!(pub grammar);

fn main() {
    let input = std::fs::read_to_string("test.hl").unwrap();
    let source = input.as_str();

    let start = std::time::Instant::now();
    let parser = grammar::RootStmntsParser::new();
    let ast = parser.parse(source, scanner::Asi::lex(source)).unwrap();
    let end = start.elapsed();

    dbg!(ast);
    println!("Parsed in {}ms", end.as_nanos() as f64 / 1_000_000f64);
}
