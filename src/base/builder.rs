use std::collections::{VecDeque, HashMap};
use crate::types::types::Type;

#[derive(Clone, Debug)]
pub struct Builder {
	pub instructions: VecDeque<String>,
	pub instruction_pointer: usize,
	pub data: HashMap<usize, VecDeque<Type>>
}

impl Builder {

	pub fn new() -> Builder {
		Builder {
			instructions: VecDeque::new(),
			instruction_pointer: 0,
			data: HashMap::new()
		}
	}

	pub fn add_instruction(&mut self, instruction: &str) {
		self.instruction_pointer += 1;
		self.instructions
			.push_back(instruction.to_string())
	}

	pub fn add_data(&mut self, data: Type) {
		if let Some(data_vec) = self.data.get_mut(&self.instruction_pointer) {
			data_vec.push_back(data);
		} else {
			let mut data_vec = VecDeque::new();
			data_vec.push_back(data);
			self.data.insert(self.instruction_pointer - 1, data_vec);
		}
	}
}
