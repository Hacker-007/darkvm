use crate::base::code::Code;
use crate::base::builder::Builder;
use crate::base::stack::Stack;
use crate::base::instruction::Instruction;
use crate::types::types::Type;

#[derive(Clone, Debug)]
pub struct Machine {
	code: Code,
	operand_stack: Stack<Type>
}

impl Machine {

	/// Constructs a new Machine object
    /// instructions: String from REPL or file
	pub fn new(builder: Builder) -> Machine {
		Machine {
			code: Code::from(builder),
			operand_stack: Stack::new(),
		}
	}

	/// Runs all of the instructions
    /// Returns prematurely if an Err occurs
	pub fn run(&mut self) -> Result<i32, String> {
		while !self.code.is_finished() {
			if let Err(e) = self.execute_next_instruction() {
				return Err(e);
			}
		}

		Ok(0)
	}

	/// Internal method to execute one instruction and returns the result of the execution
	pub fn execute_next_instruction(&mut self) -> Result<Type, String> {
		// Matches the next instruction and performs the necessary actions
		match self.code.get_next_instruction() {
			Ok(instruction) => self.perform_instruction(instruction),
			Err(e) => Err(e)
		}
	}

	pub fn perform_instruction(&mut self, instruction: Instruction) -> Result<Type, String> {
		match instruction {
			Instruction::Push => self.push(),
			Instruction::Pop => self.pop(),

			Instruction::Add => {
				let value = self.add()?;
				self.operand_stack.push(value)
			},
			Instruction::Subtract => {
				let value = self.subtract()?;
				self.operand_stack.push(value)
			},
			Instruction::Multiply => {
				let value = self.multiply()?;
				self.operand_stack.push(value)
			},
			Instruction::Divide => {
				let value = self.divide()?;
				self.operand_stack.push(value)
			},

			Instruction::Jump => self.jump(),
			Instruction::JumpRelative => self.relative_jump(),
			Instruction::JumpIfTrue => {
				let value = self.operand_stack
								.peek()
								.ok_or("Tried To Check If The Next Element Is True But No Elements Are In The Stack.".to_string())?;

				if value.is_truthy() {
					return self.jump();
				}

				Ok(Type::Number(0.0.into()))
			},
			Instruction::JumpIfFalse => {
				let value = self.operand_stack
								.peek()
								.ok_or("Tried To Check If The Next Element Is True But No Elements Are In The Stack.".to_string())?;

				if !value.is_truthy() {
					return self.jump();
				}

				Ok(Type::Number(0.0.into()))
			},

			Instruction::Print => self.print(),

			Instruction::Unknown(instruction) => Err(format!("Unknown Instruction {} Used.", instruction)),
		}
	}

	fn push(&mut self) -> Result<Type, String> {
		let mut data = self.code.get_data()?;
		match data {
			Type::String(ref string) if Instruction::check_if_instruction(string.value.as_str()) => {
				data = self.perform_instruction(Instruction::from(string.value.clone()))?;
			},
			_ => {}
		}

		self.operand_stack.push(data)
	}

	fn pop(&mut self) -> Result<Type, String> {
		let value = self.operand_stack.pop();
		value
	}

	fn add(&mut self) -> Result<Type, String> {
		self.operand_stack
			.pop()?
			.add(self.operand_stack.pop()?)
	}

	fn subtract(&mut self) -> Result<Type, String> {
		self.operand_stack
			.pop()?
			.subtract(self.operand_stack.pop()?)
	}

	fn multiply(&mut self) -> Result<Type, String> {
		self.operand_stack
			.pop()?
			.multiply(self.operand_stack.pop()?)
	}

	fn divide(&mut self) -> Result<Type, String> {
		self.operand_stack
			.pop()?
			.divide(self.operand_stack.pop()?)
	}

	fn jump(&mut self) -> Result<Type, String> {
		self.code.jump(&mut self.clone())
	}

	fn relative_jump(&mut self) -> Result<Type, String> {
		self.code.relative_jump(&mut self.clone())
	}

	fn print(&mut self) -> Result<Type, String> {
		let mut data = self.code.get_data()?;
		match data {
			Type::String(ref string) if Instruction::check_if_instruction(string.value.as_str()) => {
				data = self.perform_instruction(Instruction::from(string.value.clone()))?;
			},
			_ => {}
		}

		println!("{:#?}", data);
		Ok(Type::Number(0.into()))
	}
}