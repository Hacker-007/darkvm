pub trait Operand : Clone {
	fn add(&self, other: Self) -> Result<Self, String>;
	fn subtract(&self, other: Self) -> Result<Self, String>;
	fn multiply(&self, other: Self) -> Result<Self, String>;
	fn divide(&self, other: Self) -> Result<Self, String>;
	fn equals(&self, other: Self) -> bool;
	fn convert(value: &str) -> Result<Self, String>;
}