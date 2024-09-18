use swear_parser::{Expression, ObjectLiteral, ObjectSymbol, TopLevelItem};

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Operations {
	PushObject(ObjectLiteral),
	ConvertObject(ObjectSymbol),
	PushIdentifier(String),
	RegisterObject(String),
	RegisterCallback {
		ident: String,
		parameters: Vec<String>,
		expr: Expression,
	},
	RegisterBlueprint {
		ident: String,
		expr: Expression,
	},
	ExCallback {
		method: bool,
		callback: String,
		parameters: usize,
	},
	PushContext(Expression),
	PopContext,
	ConstructDynamicObject,
}
