extern crate getsys;

use getsys::{Cpu, PerCpu};

fn main() {
    let x = if Cpu::turbo() == true { "enabled" } else { "disabled" };
    let y = Cpu::temp();
    let z = Cpu::perc(std::time::Duration::from_millis(200));
    println!("Turbo boost is: {}", x);
    println!("Average temperature: {} °C", y);
    println!("Average cpu percentage: {:.2}%", z);

    /* get vector of values */
    let freq = PerCpu::freq();
    let gov  = PerCpu::governor();
    let driv = PerCpu::driver();

    let mut f = freq.iter();
    let mut g = gov.iter();
    let mut d = driv.iter();

    for i in 0..freq.len() {
        println!("CPU{} {} {} {}", i,
                 f.next().unwrap(),
                 g.next().unwrap(),
                 d.next().unwrap(),
                 );
    }
}
