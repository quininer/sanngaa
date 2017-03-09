#[macro_use] extern crate html5ever_atoms;
extern crate tendril;
extern crate xml5ever;
extern crate kuchiki;

pub mod parser;

pub use kuchiki::traits;
pub use parser::{ parse_xml, parse_xml_with_options };
