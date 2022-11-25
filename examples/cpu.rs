extern crate getsys;

use getsys::Cpu;

fn main() {
    let x = if Cpu::turbo() == true { "enabled" } else { "disabled" };
    let y = Cpu::cores();
    println!("Turbo boost is: {}", x);
    println!("Number of cores is: {}", y);
}
