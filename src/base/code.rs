use crate::base::instruction::Instruction;
use crate::base::builder::Builder;
use crate::types::types::Type;

use std::collections::{ HashMap, VecDeque };
use crate::base::machine::Machine;

#[derive(Clone, Debug)]
pub struct Code {
	pub instructions: VecDeque<String>,
	pub instruction_pointer: usize,
	pub data: HashMap<usize, VecDeque<Type>>
}

impl Code {

	pub fn get_next_instruction(&mut self) -> Result<Instruction, String> {
		self.instruction_pointer += 1;
		self.instructions
			.get(self.instruction_pointer - 1)
			.ok_or("Tried To Get Next Instruction When There Are None Left.".to_string())
			.map(|instruction| Instruction::from(instruction.clone()))
	}

	pub fn get_data(&mut self) -> Result<Type, String> {
		if let Some(data_vec) = self.data.get_mut(&(self.instruction_pointer- 1)) {
			return data_vec.pop_front().ok_or("The Instruction Was Not The Correct Numbers Of Parameters.".to_string());
		} else {
			return Err(format!("Unable To Find Any Data At Location {}.", self.instruction_pointer));
		}
	}

	pub fn jump(&mut self, machine: &mut Machine) -> Result<Type, String> {
		let mut new_location = self.get_data()?;
		match new_location {
			Type::String(ref string) if Instruction::check_if_instruction(string.value.as_str()) => {
				new_location = machine.perform_instruction(Instruction::from(string.value.clone()))?;
				match new_location {
					Type::Number(number) => {
						self.instruction_pointer = number.value as usize;
						return Ok(Type::Number(number.clone()));
					},
					_ => Err("The Jump Instruction Requires A Number.".to_string())
				}
			}
			Type::Number(number) => {
				self.instruction_pointer = number.value as usize;
				return Ok(Type::Number(number));
			},
			_ => Err("The Jump Instruction Requires A Number.".to_string())
		}
	}

	pub fn relative_jump(&mut self, machine: &mut Machine) -> Result<Type, String> {
		let mut new_location = self.get_data()?;
		match new_location {
			Type::String(ref string) if Instruction::check_if_instruction(string.value.as_str()) => {
				new_location = machine.perform_instruction(Instruction::from(string.value.clone()))?;
				match new_location {
					Type::Number(number) => {
						self.instruction_pointer += number.value as usize - 1;
						return Ok(Type::Number(number.clone()));
					},
					_ => Err("The Relative Jump Instruction Requires A Number.".to_string())
				}
			}
			Type::Number(number) => {
				self.instruction_pointer += number.value as usize - 1;
				return Ok(Type::Number(number));
			},
			_ => Err("The Relative Jump Instruction Requires A Number.".to_string())
		}
	}

	pub fn is_finished(&self) -> bool {
		self.instruction_pointer == self.instructions.len()
	}
}

impl From<Builder> for Code {

	fn from(builder: Builder) -> Code {
		Code {
			instructions: builder.instructions,
			instruction_pointer: 0,
			data: builder.data,
		}
	}
}

