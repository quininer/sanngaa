extern crate markup5ever;
extern crate tendril;
extern crate xml5ever;
extern crate kuchiki;

mod parser;

// re-export kuchiki
pub use kuchiki::{
    traits, iter,
    Attributes, NodeDataRef, Selectors,
    NodeRef, Node, NodeData, ElementData, Doctype, DocumentData
};
pub use parser::{ ParseOpts, parse_xml, parse_xml_with_options };
