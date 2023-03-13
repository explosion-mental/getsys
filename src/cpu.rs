//! CPU related functions
//! # Examples
//! ```
#![doc = include_str!("../examples/cpu.rs")]
//! ```
//! - [ ] Windows turbo boost support
//! reference: <https://answers.microsoft.com/en-us/windows/forum/all/windows-10-unable-to-access-complete-power-options/4aa305ed-d788-4356-b7b5-9b752fdd0944>
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::fmt;

/// Returns true if the turbo boost is enabled, false if it is disabled or not supported.
/// NOTE: use [`try_turbo()`]
#[deprecated(since="1.1.0", note="please use `try_turbo()` with `TurboState` instead.")]
pub fn turbo() -> bool {
    let intelpstate = "/sys/devices/system/cpu/intel_pstate/no_turbo";
    let cpufreq = "/sys/devices/system/cpu/cpufreq/boost";
    let path = if Path::new(intelpstate).exists() {
        intelpstate
    } else if Path::new(cpufreq).exists() {
        cpufreq
    } else {
        return false;
    };


    if fs::read_to_string(path).expect("sysfs file shoudln't return an error").trim() == "1" {
        return true
    }

    false
}

/// Possible values for turbo
pub enum TurboState {
    On,
    Off,
    NotSupported,
}

/// Add a simple `Display` for [`TurboState`]
impl fmt::Display for TurboState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::On           => write!(f, "Enabled"),
            Self::Off          => write!(f, "Disabled"),
            Self::NotSupported => write!(f, "Not Supported"),
        }
    }
}

/// Same as `turbo()` but takes into account `NotSupported` machines.
pub fn try_turbo() -> TurboState {
    let intelpstate = "/sys/devices/system/cpu/intel_pstate/no_turbo";
    let cpufreq = "/sys/devices/system/cpu/cpufreq/boost";
    let path = if Path::new(intelpstate).exists() {
        intelpstate
    } else if Path::new(cpufreq).exists() {
        cpufreq
    } else {
        return TurboState::NotSupported;
    };


    if fs::read_to_string(path).expect("/sys fs should be avaliable for reading").trim() == "1" {
        TurboState::On
    } else {
        TurboState::Off
    }
}

/// Average CPU usage as a f64 value percentage from 0% - 100%. It takes as a parameter the
/// amount of time to sleep in between reads, since to get an average of the usage, we will need
/// to compare values passed an interval of time.
pub fn perc(sleeptime: std::time::Duration) -> f64 {
    let getstats = || {
        let mut firstline = String::new();
        std::io::BufReader::new(
            File::open("/proc/stat").expect("'/proc/stat' should exist.")
            )
            .read_line(&mut firstline).
            expect("/proc fs should be avaliable for reading.");
        let mut s = firstline.split_ascii_whitespace();
        // cpu user nice system idle iowait irq softirq
        //        0    1      2    3      4   5       6
        let _cpu     = s.next(); //ignore the "cpu" word
        let user     = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let nice     = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let system   = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let idle     = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let iowait   = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let irq      = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");
        let softirq  = s.next().unwrap_or("0").parse::<f64>().expect("should always be a number");

        (user, nice, system, idle, iowait, irq, softirq)
    };

    // first iteration
    let (user, nice, sys, idle, wait, irq, sirq) = getstats();

    // sleep
    std::thread::sleep(sleeptime);

    // agane..
    let (user2, nice2, sys2, idle2, wait2, irq2, sirq2) = getstats();

    // Compare values and get an average
    if user2 == 0.0 { return 0.0 } // No Change

    let sum = (user2 + nice2 + sys2 + idle2 + wait2 + irq2 + sirq2) -
              (user  + nice  + sys  + idle  + wait  + irq  + sirq );

    if sum == 0.0 { return 0.0 } // No Change

    100.0 *
        ((user2 + nice2 + sys2 + irq2 + sirq2) - (user  + nice  + sys  + irq  + sirq))
    / sum
}

/// Average system temperature
pub fn temp() -> u32 {
    //TODO check for other paths
    let thermal = "/sys/class/thermal/thermal_zone0/temp";
    let hwmon = "/sys/class/hwmon/hwmon0/device/temp1_input";

    let path = if Path::new(thermal).exists() {
        thermal
    } else if Path::new(hwmon).exists() {
        hwmon
    } else { /* Err Couldn't get temp */
        return 0;
    };

    fs::read_to_string(path).expect("reason").trim().parse::<u32>().unwrap() / 1000_u32
}
