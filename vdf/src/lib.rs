#[macro_use]
extern crate nom;

mod reader;
pub use reader::Reader;

pub mod parser;
pub use parser::Event;
