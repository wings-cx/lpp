use std::fmt::Write;

use ansi_term::Color::{Blue, Fixed, Green, Red, White};
use tree_sitter::{Node, TreeCursor};
use typed_builder::TypedBuilder;

use crate::Failable;

/// Options used when creating a [`TreePrinter`].
#[derive(TypedBuilder, Default)]
pub struct TreePrinterOptions<'source> {
    source: &'source str,

    #[builder(default = false)]
    byte_offsets: bool,

    #[builder(default = false)]
    positions: bool,

    #[builder(default = false)]
    sources: bool,

    #[builder(default = false)]
    fields: bool,
}

/// Used to print the contents of a [`tree_sitter::Tree`] with different formatting options.
pub struct TreePrinter<'source> {
    indent: usize,
    options: TreePrinterOptions<'source>,
}

impl<'source> TreePrinter<'source> {
    /// Returns a [`TreePrinter`] with the [`TreePrinterOptions`] passed.
    pub fn new(options: TreePrinterOptions<'source>) -> Self {
        Self { indent: 0, options }
    }

    /// Prints the [`tree_sitter::Node`] passed. Indentation is reset everytime this is called.
    pub fn invoke(&mut self, node: Node) -> Failable<()> {
        self.indent = 0;

        let mut cursor = node.walk();
        self.invoke_inner(&mut cursor)?;

        Ok(())
    }

    fn invoke_inner(&mut self, cursor: &mut TreeCursor) -> Failable<()> {
        let mut index: u32 = 0;

        loop {
            let node = cursor.node();

            if node.is_named() || node.is_error() {
                let mut message = String::new();

                write!(message, "{}", " ".repeat(self.indent))?; // indent

                if self.options.fields && node.parent().is_some() {
                    let parent = node.parent().unwrap();
                    let field_name = parent.field_name_for_child(index);

                    if let Some(field_name) = field_name {
                        write!(message, "{}: ", Blue.paint(format!("'{}'", field_name)),)?;
                    }
                }

                if node.is_error() {
                    write!(message, "{}", Red.bold().paint("error"))?;
                } else {
                    write!(message, "{}", White.bold().paint(node.kind()))?;
                }

                if self.options.positions {
                    write!(
                        message,
                        " {}",
                        Fixed(8).paint(format!(
                            "<{}, {}>",
                            node.start_position(),
                            node.end_position(),
                        )),
                    )?;
                }

                if self.options.byte_offsets {
                    write!(
                        message,
                        " {}",
                        Fixed(8).paint(format!(
                            "[{}, {}]",
                            node.start_position(),
                            node.end_position(),
                        )),
                    )?;
                }

                write!(message, ": ")?;

                if self.options.sources && node.parent() != None {
                    write!(
                        message,
                        "{}",
                        Green.paint(format!(
                            "\"{}\"",
                            node.utf8_text(self.options.source.as_bytes())?.trim(),
                        )),
                    )?;
                }

                println!("{}", message);
            }

            if cursor.goto_first_child() {
                self.indent += 2;
                self.invoke_inner(cursor)?;
                self.indent -= 2;
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }

            index += 1;
        }

        Ok(())
    }
}

pub fn compare_tree(expected: Node, actual: Node) -> bool {


    true
}
