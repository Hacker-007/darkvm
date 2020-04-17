use crate::types::number::NumberType;
use crate::types::operand::Operand;
use crate::types::boolean::BooleanType;
use crate::types::string::StringType;

#[derive(Clone, Debug)]
pub enum Type {
	Number(NumberType),
	Boolean(BooleanType),
	String(StringType),
}

impl Type {

	pub fn add(&self, other: Type) -> Result<Type, String> {
		match self.clone() {
			Type::Number(number) => {
				match other {
					Type::Number(other_number) => {
						Ok(Type::Number(number.add(other_number)?))
					},
					_ => Err("Addition Can Only Be Performed On Two Numbers.".to_string())
				}
			},
			_ => {
				Err("Addition Can Only Be Performed On Two Numbers.".to_string())
			}
		}
	}

	pub fn subtract(&self, other: Type) -> Result<Type, String> {
		match self.clone() {
			Type::Number(number) => {
				match other {
					Type::Number(other_number) => {
						Ok(Type::Number(number.subtract(other_number)?))
					},
					_ => Err("Addition Can Only Be Performed On Two Numbers.".to_string())
				}
			},
			_ => {
				Err("Addition Can Only Be Performed On Two Numbers.".to_string())
			}
		}
	}

	pub fn multiply(&self, other: Type) -> Result<Type, String> {
		match self.clone() {
			Type::Number(number) => {
				match other {
					Type::Number(other_number) => {
						Ok(Type::Number(number.multiply(other_number)?))
					},
					_ => Err("Addition Can Only Be Performed On Two Numbers.".to_string())
				}
			},
			_ => {
				Err("Addition Can Only Be Performed On Two Numbers.".to_string())
			}
		}
	}

	pub fn divide(&self, other: Type) -> Result<Type, String> {
		match self.clone() {
			Type::Number(number) => {
				match other {
					Type::Number(other_number) => {
						Ok(Type::Number(number.divide(other_number)?))
					},
					_ => Err("Addition Can Only Be Performed On Two Numbers.".to_string())
				}
			},
			_ => {
				Err("Addition Can Only Be Performed On Two Numbers.".to_string())
			}
		}
	}

	pub fn convert(value: &str) -> Result<Type, String> {
		if let Ok(number) = value.parse::<f32>() {
			return Ok(Type::Number(NumberType::new(number)));
		}

		if let Ok(boolean) = value.parse::<bool>() {
			return Ok(Type::Boolean(BooleanType::new(boolean)));
		}

		Ok(Type::String(StringType::new(value)))
	}

	pub fn is_truthy(&self) -> bool {
		match self {
			Type::Number(number) => !number.equals(0.0.into()),
			Type::Boolean(boolean) => boolean.value,
			Type::String(string) => !string.value.is_empty(),
		}
	}
}