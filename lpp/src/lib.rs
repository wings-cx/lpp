mod ast;
mod error;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub language);
