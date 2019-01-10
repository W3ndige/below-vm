pub mod memory;
pub mod cpu;
pub mod vm;

use vm::VM;

fn main() {
    let mut vm = VM::new();
    vm.fetch_and_dispatch();
}
