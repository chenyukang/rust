//! The main parser interface.
//#![feature(pointer_byte_offsets)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(if_let_guard)]
#![feature(iter_intersperse)]
#![feature(let_chains)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![recursion_limit = "256"]
#![allow(internal_features)]
#![allow(invalid_doc_attributes)]

extern crate tracing;

use rustc_errors::{DiagnosticMessage, SubdiagnosticMessage};
use rustc_fluent_macro::fluent_messages;

pub const MACRO_ARGUMENTS: Option<&str> = Some("macro arguments");

#[macro_use]
//pub mod parser;
//use parser::{make_unclosed_delims_error, Parser};
//pub mod lexer;
//pub mod validate_attr;

mod errors;

fluent_messages! { "../messages.ftl" }
