extern crate getsys;

use getsys::Cpu;
use getsys::Cpu::TurboState;

fn main() {
    let turbo = match Cpu::try_turbo() {
        TurboState::On => "enabled",
        TurboState::Off => "enabled",
        TurboState::NotSupported => "This machine doesn't support turbo boost",

    };

    println!("Turbo boost is: {}", turbo);
    println!("Average temperature: {} °C", Cpu::temp());
    println!("Average cpu percentage: {:.2}%", Cpu::perc(std::time::Duration::from_millis(200)));
}
