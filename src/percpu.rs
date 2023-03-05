//! Per cpu information, rather than average or aggregates like Cpu
use std::fs;

use glob::glob;

///TODO there is more per cpu information in /proc/stat/
///TODO switch glob with num_cpus
/// Reference: https://www.kernel.org/doc/html/v4.14/admin-guide/pm/cpufreq.html


/// Returns a vector of strings that represents the scaling governor the
/// respective cpu is using, from cpu 0 to cpu X.
/// You can view avaliable governors with:
/// cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
///
/// The vector is as large as the number of cpus.
pub fn governor() -> Vec<String> {
    let mut govs: Vec<String> = Vec::new();

    //governor
    for entry in glob("/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_governor").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => govs.push(
                fs::read_to_string(path).expect("sysfs file shoudln't return an error").trim().to_string()
            ),

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }

    govs
}

/// Returns a vector of strings that represents the current frequency in kHz
/// the respective cpu is using, from cpu 0 to cpu X.
/// The vector is as large as the number of cpus.
pub fn freq() -> Vec<String> {
    let mut govs: Vec<String> = Vec::new();
    //TODO use a more precise type than a String

    //frequency
    for entry in glob("/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_cur_freq").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => govs.push(
                fs::read_to_string(path).expect("sysfs file shoudln't return an error").trim().to_string()
            ),

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }

    govs
}

/// Returns a vector of strings that represents the driver, or _policy_, the
/// respective cpu is using, from cpu 0 to cpu X.
/// The vector is as large as the number of cpus.
pub fn driver() -> Vec<String> {
    let mut govs: Vec<String> = Vec::new();

    //driver
    for entry in glob("/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_driver").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => govs.push(
                fs::read_to_string(path).expect("sysfs file shoudln't return an error").trim().to_string()
            ),

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }

    govs
}
