extern crate getsys;

use getsys::PerCpu;

fn main() {
    // Vector of values
    let freq = PerCpu::freq();
    let gov  = PerCpu::governor();
    let driv = PerCpu::driver();

    let mut f = freq.iter();
    let mut g = gov.iter();
    let mut d = driv.iter();

    for i in 0..freq.len() {
        println!("CPU{} {} {} {}",
            i,
            f.next().unwrap(),
            g.next().unwrap(),
            d.next().unwrap(),
        );
    }
}
