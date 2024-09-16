use swear_parser::{ObjectLiteral, ObjectSymbol, TopLevelItem};

#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum Operations {
	PushObject(ObjectLiteral),
	ConvertObject(ObjectSymbol),
	PushIdentifier(String),
	RegisterObject(String),
	RegisterCallback(String),
	RegisterBlueprint(String),
	CallCallback {
		method: bool,
		callback: String,
		parameters: usize,
	},
	PushContext(Vec<TopLevelItem>),
	PopContext,
}
