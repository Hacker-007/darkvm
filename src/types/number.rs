use crate::types::operand::Operand;

#[derive(Clone, Debug)]
pub struct NumberType {
	pub value: f32
}

impl NumberType {

	pub fn new(value: f32) -> NumberType {
		NumberType {
			value
		}
	}
}

impl Operand for NumberType {

	fn add(&self, other: Self) -> Result<Self, String> {
		Ok(NumberType::new(self.value + other.value))
	}

	fn subtract(&self, other: Self) -> Result<Self, String> {
		Ok(NumberType::new(self.value - other.value))
	}

	fn multiply(&self, other: Self) -> Result<Self, String> {
		Ok(NumberType::new(self.value * other.value))
	}

	fn divide(&self, other: Self) -> Result<Self, String> {
		if other.value == 0.0 {
			return Err("Tried To Divide By 0.".to_string());
		}

		Ok(NumberType::new(self.value / other.value))
	}

	fn equals(&self, other: Self) -> bool {
		self.value == other.value
	}

	fn convert(value: &str) -> Result<Self, String> {
		match value.parse::<f32>() {
			Ok(value) => Ok(NumberType { value }),
			Err(_) => Err(format!("The Value '{}' Could Not Be Formed Into An Operand.", value))
		}
	}
}

impl From<i32> for NumberType {

	fn from(value: i32) -> NumberType {
		NumberType::new(value as f32)
	}
}

impl From<f32> for NumberType {

	fn from(value: f32) -> NumberType {
		NumberType::new(value)
	}
}