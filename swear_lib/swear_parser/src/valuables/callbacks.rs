use super::*;

#[derive(Debug, Clone)]
pub struct MethodCallback {
	pub target: Box<Option<Valuable>>,
	pub id: String,
	pub parameters: Vec<Valuable>,
}

impl MethodCallback {
	pub fn new(target: Option<Valuable>, id: &str, parameters: Vec<Valuable>) -> Self {
		MethodCallback {
			target: Box::new(target),
			id: id.into(),
			parameters,
		}
	}
}
