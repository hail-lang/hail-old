pub mod ast;
pub mod scanner;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);