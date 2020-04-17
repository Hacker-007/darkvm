#[derive(Clone, Debug)]
pub struct Stack<T: Clone> {
	stack: Vec<T>
}

impl<T: Clone> Stack<T> {

	/// Constructs a new Stack object
	pub fn new() -> Stack<T> where T: Clone {
		Stack {
			stack: vec![]
		}
	}

	/// Pushes the element specified
	/// Returns the value that was provided
	pub fn push(&mut self, value: T) -> Result<T, String> {
		self.stack
			.push(value.clone());
		Ok(value)
	}

	/// Pops the next element and returns the element
	/// Returns an Err if the stack is empty
	pub fn pop(&mut self) -> Result<T, String> {
		self.stack
			.pop()
			.ok_or("Can Not Pop From An Empty Stack.".to_string())
	}

	pub fn peek(&mut self) -> Option<T> {
		self.stack
			.get(0)
			.map(|value| value.clone())
	}
}