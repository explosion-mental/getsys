extern crate getsys;

use getsys::Battery;

fn main() {
    let ischarging = Battery::ac_status();
    let bat = Battery::perc();
    println!("AC adapter status: {}", if ischarging { "charging" } else { "disconected" });
    println!("Remaining battery: {}%", bat);
}
