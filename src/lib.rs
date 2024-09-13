#![allow(dead_code)]
#![feature(trait_upcasting)]
#![warn(missing_docs)]

//! A hacking-friendly [ECMAScript](https://tc39.es/ecma262/multipage/) engine.

pub mod completions;
pub mod core;
pub mod errors;
