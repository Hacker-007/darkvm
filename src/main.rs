use crate::base::builder::Builder;
use crate::base::machine::Machine;
use crate::types::types::Type;

use std::{env, io};
use std::fs;
use std::io::Write;

mod base;
mod types;

fn main() {
	run();
}

fn run() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 {
		match run_repl() {
			Ok(success_code) => println!("Session Succeeded With A Code Of {}.", success_code),
			Err(error_message) => println!("An Error Occurred: {}", error_message)
		}
	} else {
		match run_file(args.get(1).unwrap().as_str()) {
			Ok(success_code) => println!("Code Succeeded With A Code Of {}.", success_code),
			Err(error_message) => println!("An Error Occurred: {}", error_message)
		}
	}
}

fn run_repl() -> Result<i32, String>{
	// Constructs the buffer where line is stored
	let mut buffer = String::new();
	// Prompts the user
	print!("dark > ");
	let _ = io::stdout().flush();
	let stdin = io::stdin();
	while !buffer.eq(&"!quit".to_string()) {
		let mut builder = Builder::new();
		// Reads the input from the user
		match stdin.read_line(&mut buffer) {
			Err(_) => {
				println!("Error Occurred During Interaction.");
			},
			_ => {}
		}

		let mut commands = buffer.split_whitespace();
		let mut parse_error = false;
		while let Some(command) = commands.next() {
			let command = command.to_lowercase();
			let instruction = command.as_str();
			match instruction {
				"pop" |
				"add" |
				"sub" |
				"mul" |
				"div" => builder.add_instruction(instruction),

				"push"  |
				"jmp"   |
				"rjmp"  |
				"jmpt"  |
				"jmpf"  |
				"print" => {
					builder.add_instruction(instruction);
					let parameter = Type::convert(
						commands.next()
								.ok_or(
									format!(
										"The Instruction '{}' Required 1 Parameter But There Was No More Data To Read.",
										instruction
									)
								)?
					)?;

					builder.add_data(parameter);
				},

				_ => {
					println!("The Instruction '{}' Does Not Exist.", instruction);
					parse_error = true;
					break;
				}
			}
		}

		if !parse_error {
			let mut machine = Machine::new(builder);
			let result = machine.run();
			match result {
				Ok(_) => println!("{:#?}", machine),
				Err(e) => println!("{}", e),
			}
		}

		// Resets the buffer and prompts the user
		buffer.clear();
		print!("dark > ");
		let _ = io::stdout().flush();
	}

	Ok(0)
}

fn run_file(file_path: &str) -> Result<i32, String> {
	let contents = fs::read_to_string(file_path).map_err(|_| format!("Error With File Located At {}.", file_path))?;
	let mut contents = contents.split_whitespace();
	let mut builder = Builder::new();
	let mut parse_error = false;
	while let Some(command) = contents.next() {
		let command = command.to_lowercase();
		let instruction = command.as_str();
		match instruction {
			"pop" |
			"add" |
			"sub" |
			"mul" |
			"div" => builder.add_instruction(instruction),

			"push"  |
			"jmp"   |
			"rjmp"  |
			"jmpt"  |
			"jmpf"  |
			"print" => {
				builder.add_instruction(instruction);
				let parameter = Type::convert(
					contents.next()
						.ok_or(
							format!(
								"The Instruction '{}' Required 1 Parameter But There Was No More Data To Read.",
								instruction
							)
						)?
				)?;

				builder.add_data(parameter);
			},

			_ => {
				println!("The Instruction '{}' Does Not Exist.", instruction);
				parse_error = true;
				break;
			}
		}
	}

	let mut machine = Machine::new(builder);
	let result = machine.run();
	if result.is_ok() && !parse_error {
		println!("{:#?}", machine);
		return result;
	}

	if result.is_err() {
		return result;
	}

	Err("Invalid Instruction.".to_string())
}

fn tokenize(string: String) -> Vec<String> {
	let mut contents = vec![];
	let mut current = String::new();
	let mut chars = string.chars();
	for character in chars {
		current += &character.to_string();
	}

	contents
}