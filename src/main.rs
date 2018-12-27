pub mod instructions;
pub mod vm;

use vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.run();
}
