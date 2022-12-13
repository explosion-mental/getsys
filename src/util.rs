/// CPU related utility functions that are used in Cpu in the backend
use std::path::Path;

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

