/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg_attr(feature = "cargo-clippy", allow(match_ref_pats))]
#![feature(proc_macro)]
#![recursion_limit = "512"]

#[macro_use]
extern crate combine;
extern crate itertools;
#[macro_use]
extern crate quote;
extern crate rand;
extern crate rsx_shared;
extern crate self_tokenize_macro;
extern crate self_tokenize_trait;

mod parse_attributes_types;
mod parse_attributes;
mod parse_children_types;
mod parse_children;
mod parse_elements_types;
mod parse_elements;
mod parse_external_placeholders;
mod parse_external_types;
mod parse_external;
mod parse_js_types;
mod parse_js;
mod parse_misc;
mod parse_rsx;
mod parse_rust_types;
mod parse_rust;
mod tokenize_attributes;
mod tokenize_children;
mod tokenize_elements;
mod tokenize_external;

#[cfg(test)]
mod test_helpers;

pub mod types {
    pub use parse_attributes_types::*;
    pub use parse_children_types::*;
    pub use parse_elements_types::*;
    pub use parse_external_placeholders::*;
    pub use parse_external_types::*;
    pub use parse_js_types::*;
    pub use parse_rust_types::*;
}

use combine::{ParseError, Parser};
use combine::combinator::parser;

pub fn parse(s: &str) -> Result<(types::RSXElement, &str), ParseError<&str>> {
    parser(parse_rsx::rsx_element_ignoring_ws).parse(s)
}
