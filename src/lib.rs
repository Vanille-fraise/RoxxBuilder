#![feature(array_methods)]

pub mod builder;
mod web;

extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate num_derive;
extern crate serde_json;
extern crate serde_big_array;