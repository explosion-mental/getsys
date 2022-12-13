extern crate getsys;

use getsys::{Cpu, PerCpu};

fn main() {
    println!("Turbo boost is: {}",
             if Cpu::turbo() == true { "enabled" } else { "disabled" }
             );
    println!("Average temperature: {} °C", Cpu::temp());
    println!("Getting cpu percentage...");
    println!("Average cpu percentage: {:.2}%",
             Cpu::perc(std::time::Duration::from_millis(200))
             );

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
