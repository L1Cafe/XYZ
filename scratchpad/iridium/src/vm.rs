pub struct VM {
	registers: [i32; 32],
	pc: usize,
	program: Vec<u8>
}

impl VM {
	pub fn new() -> VM {
		VM {
			registers: [0; 32]
			program: vec![],
			pc: 0,
		}
	}
	pub fn run(&mut self) {
		loop {
			// If our program counter has exceeded the length of the program itself, something has
			// gone awry
			if self.pc >= self.program.len() {
				break;
			}
			match self.decode_opcode() {
				Opcode::HLT => {
					println!("HLT encountered");
					return;
				},
				_ => {
					println!("Unrecognized opcode found! Terminating!");
					return;
				}
			}
		}
	}
	fn decode_opcode(&mut self) -> Opcode {
		let opcode = Opcode::from(self.program[self.pc]);
		self.pc += 1;
		return opcode;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_create_vm() {
		let test_vm = VM::new();
		assert_eq!(test_vm.registers[0], 0)
	}
}