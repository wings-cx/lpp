use lpp::{
    util::{TreePrinter, TreePrinterOptions},
    Error,
};
use std::{fs::File, io::Read};

use clap::Args;
use lpp::Failable;
use tree_sitter::Parser;

/// Contains the arguments required for [`generate_cst`].
#[derive(Args)]
pub struct GenerateCst {
    path: String,

    #[clap(
        short = 'S',
        long = "sources",
        help = "Includes the source of each terminal or non-terminal. Sources aren't included for nodes which contain '\\n' characters."
    )]
    sources: bool,

    #[clap(
        short = 'b',
        long = "byte-offsets",
        help = "Includes byte offsets into the original source file for each terminal or non-terminal."
    )]
    byte_offsets: bool,

    #[clap(
        short,
        long,
        help = "Includes row-column position pairs for each terminal or non-terminal."
    )]
    positions: bool,

    #[clap(
        short,
        long,
        help = "Includes tree-sitter fields for terminals and non-terminals which contains contain them."
    )]
    fields: bool,
}

/// Prints the concrete syntax tree of the file passed.
pub fn generate_cst(args: GenerateCst) -> Failable<()> {
    let mut file = File::open(args.path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_lpp::language())?;

    let tree = parser.parse(contents.as_str(), None);
    let tree = match tree {
        Some(tree) => tree,
        None => return Err(Error::new("failed to parse tree")),
    };

    let mut printer = TreePrinter::new(
        TreePrinterOptions::builder()
            .source(contents.as_str())
            .sources(args.sources)
            .positions(args.positions)
            .fields(args.fields)
            .build(),
    );

    printer.invoke(tree.root_node())?;

    Ok(())
}

/// Contains the arguments for [`test_cst`].
#[derive(Args)]
pub struct TestCst {
    
}

/// Runs tree-sitter-style tests, but with a more fancy output.
/// Files should be formatted like the following,
///
/// ```
/// #test assignments
///
/// #section input
/// local a = 1
///
/// #section expected
/// 
/// ```
pub fn test_cst(args: TestCst) {
    
}
