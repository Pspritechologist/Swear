#![feature(ascii_char)]
#![feature(trait_alias)]
#![feature(extract_if)]

pub mod objects;
pub mod runtime;
pub mod context;

use runtime::evaluation::Evaluable;
pub use swear_parser;

use objects::*;
use std::{collections::HashMap, fmt::Debug};
use swear_parser::*;
use context::*;
use std::cell::RefCell;

pub type Operations = Vec<TopLevelItem>;
pub type Callback = (Vec<String>, Operations);
pub trait ContextDb = context::Context + Debug;

pub fn execute_expression(mut expr: Operations, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
	let ahead = expr.extract_if(|item| {
		match item {
			TopLevelItem::Definition(Definition::Blueprint { .. }) |
			TopLevelItem::Definition(Definition::Callback { .. }) => true,
			_ => false,
		}
	});

	for item in ahead {
		match item {
			TopLevelItem::Definition(Definition::Blueprint { name, exprs }) => {
				cont.set_value(&name, ContextVar::Blueprint(exprs.contents));
			}
			TopLevelItem::Definition(Definition::Callback { name, parameters, exprs}) => {
				cont.set_value(&name, ContextVar::Callback((parameters, exprs.contents)));
			}
			// _ => continue, // I'd much rather this filter the list ahead of time, but it'll do for now.
			_ => unreachable!("Unexpected item in 'ahead of time' portion of operations: {:?}", item),
		}
	}

	let mut result: RefCell<Object> = RefCell::new(Object::default());

	for item in expr {
		match item {
			TopLevelItem::Definition(Definition::Register { name, value }) => {
				let value = value.evaluate(cont);
				cont.set_value(&name, ContextVar::Register(value));
			},
			TopLevelItem::Valuable(valuable) => {
				result = valuable.evaluate(cont);
			},
			TopLevelItem::Dropper(value) => {
				return value.map_or_else(|| RefCell::new(Object::default()), |v| v.evaluate(cont))
			},
			// _ => continue, // I'd much rather this filter the list ahead of time, but it'll do for now.
			_ => unreachable!("Unexpected item in 'runtime' portion of operations: {:?}", item),
		}
	}

	result
}
