extern crate getsys;

use getsys::Cpu;

fn main() {
    let x = Cpu::turbo();
    println!("Turbo boost is: {}", x);
}
