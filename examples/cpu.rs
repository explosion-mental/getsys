extern crate getsys;

use getsys::Cpu;

fn main() {
    let x = if Cpu::turbo() == true { "enabled" } else { "disabled" };
    let y = Cpu::cores();
    let z = Cpu::temp();
    println!("Turbo boost is: {}", x);
    println!("Number of cores is: {}", y);
    println!("Average temperature: {} °C", z);
}
