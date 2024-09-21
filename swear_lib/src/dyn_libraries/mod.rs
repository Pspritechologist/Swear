use std::{path::Path, sync::Arc};
use libloading::Library;

use crate::{context::ObjectRef, object::{self, Object}};

// Functions expected to be present in the dynamic library.
const INIT_FUNC: &[u8] = b"init\0";
type InitFunc = unsafe extern fn(&mut object::Dynamic);

pub fn load_library(path: &Path) -> Result<ObjectRef, libloading::Error> {
	let lib: Library;
	let mut object = object::Dynamic::default();

	unsafe {
		lib = Library::new(path)?;
		let func = lib.get::<InitFunc>(INIT_FUNC)?;
		func(&mut object);
	}
	
	object.src_lib = Some(Arc::new(lib));
	Ok(Object::from(object).into())
}
