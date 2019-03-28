//! parser.rs - Contains Wolfram Language parser definitions.

use pest::Parser;

/// The core Wolfram Language parser object.
#[derive(Parser)]
#[grammar = "wl.pest"]
pub struct WLParser;
