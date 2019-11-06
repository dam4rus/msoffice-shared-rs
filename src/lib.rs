#![forbid(unsafe_code)]

pub mod docprops;
pub mod drawingml;
pub mod error;
pub mod relationship;
pub mod sharedtypes;
pub mod xml;
pub mod xsdtypes;
pub mod update;

extern crate strum;
#[macro_use]
extern crate strum_macros;
