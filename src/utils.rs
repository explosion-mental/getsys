/// CPU related utility functions that are used in Cpu in the backend
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub enum TurboBoost {
    Intelp,
    CpuFreq,
    None,
}

pub fn get_turbo_path() -> TurboBoost {
    let intelpstate = "/sys/devices/system/cpu/intel_pstate/no_turbo";
    let cpufreq = "/sys/devices/system/cpu/cpufreq/boost";

    if Path::new(intelpstate).exists() {
        return TurboBoost::Intelp;
    } else if Path::new(cpufreq).exists() {
        return TurboBoost::CpuFreq;
    }

    return TurboBoost::None;
}

//read file from path and return a String with ALL the contents of it.
pub fn read_path(path: &str) -> String {
    let mut ret = String::new();

    File::open(path).expect("Cannot open file.")
        .read_to_string(&mut ret)
        .expect("Cannot read file.");

    ret
}
