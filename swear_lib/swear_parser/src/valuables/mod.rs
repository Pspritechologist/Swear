mod expressions;
mod objects;
mod callbacks;

use super::*;

pub use expressions::Expression;
pub use callbacks::MethodCallback;
pub use objects::{ObjectLiteral, ObjectConversion};

#[derive(Debug, Clone)]
pub enum Valuable {
	ObjectLiteral(ObjectLiteral),
	ObjectConversion(Box<ObjectConversion>),
	Callback(MethodCallback),
	Expression(Expression),
	Identifier(String),
}

impl From<ObjectLiteral> for Valuable {
	fn from(value: ObjectLiteral) -> Self {
		Valuable::ObjectLiteral(value)
	}
}

impl From<ObjectConversion> for Valuable {
	fn from(value: ObjectConversion) -> Self {
		Valuable::ObjectConversion(Box::new(value))
	}
}

impl From<MethodCallback> for Valuable {
	fn from(value: MethodCallback) -> Self {
		Valuable::Callback(value)
	}
}

impl From<Expression> for Valuable {
	fn from(value: Expression) -> Self {
		Valuable::Expression(value)
	}
}

impl From<String> for Valuable {
	fn from(value: String) -> Self {
		Valuable::Identifier(value)
	}
}
