#[derive(Debug, PartialEq)]
pub enum Opcode {
	HLT,
	IGL
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
	opcode: Opcode
}

// Trait
impl From<u8> for Opcode {
	fn from(v: u8) -> Self {
		match v {
			0 => return Opcode::HLT,		// HALT
			_ => return Opcode::IGL			// ILLEGAL
		}
	}
}

impl Instruction {
	pub fn new(opcode: Opcode) -> Instruction {
		Instruction {
			opcode: opcode
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
	
// 	#[test]
// 	fn test_create_vm() {
// 		let test_vm = VM::new();
// 		assert_eq!(test_vm.registers[0], 0)
// 	}
	
// 	#[test]
// 	fn test_opcode_hlt() {
// 		let mut test_vm = VM::new();
// 		let test_bytes = vec![0,0,0,0];
// 		test_vm.program = test_bytes;
// 		test_vm.run();
// 		assert_eq!(test_vm.pc, 1);
// 	}
	
// 	#[test]
// 	fn test_opcode_igl() {
// 		let mut test_vm = VM::new();
// 		let test_bytes = vec![200,0,0,0];
// 		let test_bytes2 = vec![200,100,50,0];
// 		test_vm.program = test_bytes;
// 		test_vm.run();
// 		assert_eq!(test_vm.pc, 2);
// 	}
// }