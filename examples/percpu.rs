extern crate getsys;

use getsys::PerCpu;

fn main() {
    // init the `Vec<String>`s to get an `.iter()` below
    let freq = PerCpu::freq();
    let gov  = PerCpu::governor();
    let driv = PerCpu::driver();

    // `.zip()` `.iter()` values as ((governor, driver), frequency), then `.enumerate()`
    let values = gov.iter().zip(driv.iter()).zip(freq.iter()).enumerate();

    // iterate over values and print it
    for (i, ((governor, driver), frequency)) in values {
        println!("CPU{} {} {} {}", i, frequency, driver, governor);
    }
}
