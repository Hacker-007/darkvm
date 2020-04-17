#[derive(Clone, Debug)]
pub struct StringType {
	pub value: String
}

impl StringType {

	pub fn new(value: &str) -> StringType {
		StringType {
			value: value.to_string()
		}
	}
}