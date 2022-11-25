#![allow(dead_code)]
mod cpu_utils {
    use::std::path::Path;

    pub fn get_turbo_path() -> Option<String> {
        let intelpstate = "/sys/devices/system/cpu/intel_pstate/no_turbo";
        let cpufreq = "/sys/devices/system/cpu/cpufreq/boost";

        if Path::new(intelpstate).exists() {
            return Some(intelpstate.to_string());
        } else if Path::new(cpufreq).exists() {
            return Some(cpufreq.to_string());
        } else { /* turbo boost is not supported */
            return None;
        }
    }
}

/* public dev interface */
use crate::cpu_utils::get_turbo_path;
use std::io::Read;
use std::fs::File;

pub struct Cpu {}

impl Cpu {
    pub fn turbo() -> bool {
        let mut governor = String::new();

        match get_turbo_path() {
            None => return false,
            Some(path) => { // read path
                File::open(path)
                    .expect("Cannot open file.")
                    .read_to_string(&mut governor)
                    .expect("Cannot read file.");
            },
        }

        if governor.trim() == "1" {
            return true
        } else {
            return false
        }
    }

    fn cores() -> u32 {
        8
    }

    fn perc() -> u32 {
        100
    }

    fn temp() -> u32 {
        42
    }
}

