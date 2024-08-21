use super::*;

pub trait Evaluable {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object>;
}

impl Evaluable for ObjectLiteral {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		match self {
			ObjectLiteral::Chars(chars) => RefCell::new(Chars::new(chars).into()),
			ObjectLiteral::State(state) => RefCell::new(State::new(state).into()),
			ObjectLiteral::Count(count) => RefCell::new(Count::new(count).into()),
			ObjectLiteral::Zip => RefCell::new(Zip::new().into()),
			ObjectLiteral::Deck(deck) => RefCell::new(Deck::new(deck.into_iter().map(|o| RefCell::into_inner(o.evaluate(cont))).collect()).into()),
			ObjectLiteral::Map(map) => RefCell::new(Map::new(map.into_iter().map(|(k, v)| (RefCell::into_inner(k.evaluate(cont)), RefCell::into_inner(v.evaluate(cont)))).collect()).into()),
		}
	}
}

impl Evaluable for ObjectConversion {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		match self {
			ObjectConversion::ToChars(chars) => RefCell::new(chars.evaluate(cont).into_inner().to_chars().into()),
			ObjectConversion::ToState(state) => RefCell::new(state.evaluate(cont).into_inner().to_state().into()),
			ObjectConversion::ToCount(count) => RefCell::new(count.evaluate(cont).into_inner().to_count().into()),
			ObjectConversion::ToZip(_zip) => RefCell::new(Zip::new().into()), // Converting to Zip never actually evaluates the input...
			ObjectConversion::ToDeck(deck) => RefCell::new(deck.evaluate(cont).into_inner().to_deck().into()),
			ObjectConversion::ToMap(map) => RefCell::new(map.evaluate(cont).into_inner().to_map().into()),
		}
	}
}

impl Evaluable for String {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		match cont.get_value(&self) {
			ContextVar::Register(obj) => obj.clone(),
			ContextVar::Blueprint(expr) => {
				let mut sub_cont = SubContext::new(cont.clone());
				execute_expression(expr.clone(), &mut sub_cont)
			},
			ContextVar::Callback(cb) => {
				let (_, exprs) = cb;
				let mut sub_cont = SubContext::new(cont.clone());
				execute_expression(exprs.clone(), &mut sub_cont)
			}
		}
	}
}

impl Evaluable for MethodCallback {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		let MethodCallback { id, parameters: args, target } = self;
		if let Some(target) = *target {
			todo!()
		} else {
			let cb = match cont.get_value(&id) {
				ContextVar::Callback(cb) => cb,
				o => panic!("MethodCallback::evaluate: Expected callback, found {o:?}."),
			};
			let (params, exprs) = cb;
			let args: Vec<_> = args.into_iter().map(|a| a.evaluate(cont)).collect();
			let mut sub_cont = SubContext::new(cont.clone());
			for (param, arg) in params.iter().zip(args.into_iter()) {
				sub_cont.set_value(param, ContextVar::Register(arg));
			}
			execute_expression(exprs.clone(), &mut sub_cont)
		}
	}
}

impl Evaluable for swear_parser::Expression {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		execute_expression(self.contents, cont)
	}
}

impl Evaluable for Valuable {
	fn evaluate(self, cont: &mut (impl ContextDb + Clone)) -> RefCell<Object> {
		match self {
			Valuable::ObjectLiteral(o) => o.evaluate(cont),
			Valuable::ObjectConversion(o) => o.evaluate(cont),
			Valuable::Callback(c) => c.evaluate(cont),
			Valuable::Expression(e) => e.evaluate(cont),
			Valuable::Identifier(id) => id.evaluate(cont),
		}
	}
}
