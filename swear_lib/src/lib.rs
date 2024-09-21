#![feature(fn_traits, let_chains)]

pub mod object;
pub mod context;
pub mod runtime;
pub mod dyn_libraries;

pub use swear_parser;
pub type BigNum = dashu_float::FBig<dashu_float::round::mode::Zero, 2>;
