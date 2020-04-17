#[derive(Clone, Debug)]
pub struct BooleanType {
	pub value: bool
}

impl BooleanType {

	pub fn new(value: bool) -> BooleanType {
		BooleanType {
			value
		}
	}
}

impl From<bool> for BooleanType {

	fn from(value: bool) -> BooleanType {
		BooleanType::new(value)
	}
}