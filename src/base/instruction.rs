#[derive(Debug)]
pub enum Instruction {
	Push,
	Pop,

	Add,
	Subtract,
	Multiply,
	Divide,

	Jump,
	JumpRelative,
	JumpIfTrue,
	JumpIfFalse,

	Print,

	Unknown(String)
}

impl Instruction {

	pub fn check_if_instruction(instruction: &str) -> bool {
		match Instruction::from(instruction.to_string()) {
			Instruction::Unknown(_) => false,
			_ => true
		}
	}
}

/// Implements the From trait, allowing a String to be automatically converted to an Instruction
impl From<String> for Instruction {

	fn from(instruction: String) -> Instruction {
		match instruction.to_lowercase().as_str() {
			"push"  => Instruction::Push,
			"pop"   => Instruction::Pop,

			"add"   => Instruction::Add,
			"sub"   => Instruction::Subtract,
			"mul"   => Instruction::Multiply,
			"div"   => Instruction::Divide,

			"jmp"  => Instruction::Jump,
			"rjmp" => Instruction::JumpRelative,
			"jmpt"  => Instruction::JumpIfTrue,
			"jmpf"  => Instruction::JumpIfFalse,

			"print" => Instruction::Print,

			_ => Instruction::Unknown(instruction),
		}
	}
}