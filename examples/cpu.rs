extern crate getsys;

use getsys::{Cpu, PerCpu};

fn main() {
    let x = if Cpu::turbo() == true { "enabled" } else { "disabled" };
    let y = Cpu::cores();
    let z = Cpu::temp();
    let m = Cpu::perc(1);
    println!("Turbo boost is: {}", x);
    println!("Number of cores is: {}", y);
    println!("Average temperature: {} °C", z);
    println!("Average cpu percentage: {:.2}%", m);

    for (i, e) in PerCpu::freq().iter().enumerate() {
        println!("Core {} has frequency of: {}", i, e);
    }

    println!("");

    for (i, e) in PerCpu::governor().iter().enumerate() {
        println!("Core {} uses {}", i, e);
    }

    println!("");

    for (i, e) in PerCpu::driver().iter().enumerate() {
        println!("Core {} uses driver {}", i, e);
    }
}
