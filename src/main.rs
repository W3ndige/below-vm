pub mod memory;
pub mod cpu;
pub mod opcodes;
pub mod vm;

use vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.run();
}
