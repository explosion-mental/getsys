//! # Per cpu information, rather than average or aggregates like Cpu
//! TODO there is more per cpu information in /proc/stat/
//! TODO switch glob with num_cpus? **need more tests**
//! Reference: https://www.kernel.org/doc/html/v4.14/admin-guide/pm/cpufreq.html
use std::fs;
use glob::glob;

/// possible percpu values
enum Kind {
    Governor,
    Freq,
    Driver,
}

/// Wrapper to get percpu values (avoids duplicating code). This is called by all other functions
/// on percpu. Adding more values should be easy enough
//TODO error handling? glob shouldn't return an error, if it does the path doesn't exist and values cannot be obtained..
fn get_value(n: Kind) -> Vec<String> {
    let mut data = vec![];

    let path = match n {
        Kind::Governor => "/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_governor",
        Kind::Freq     => "/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_cur_freq",
        Kind::Driver   => "/sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_driver",
    };

    for entry in glob(path).expect("glob pattern is ok") {
        let info = match entry {
            Ok(path) => fs::read_to_string(path).expect("/sys fs should be avaliable for reading.").trim().to_owned(),
            Err(e) => { eprintln!("{e}"); "Unavaliable".to_owned() },
        };
        data.push(info);
    }

    data
}

/// Returns a vector of strings that represents the scaling governor the
/// respective cpu is using, from cpu 0 to cpu X.
/// You can view avaliable governors with:
/// cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors
///
/// The vector is as large as the number of cpus.
pub fn governor() -> Vec<String> {
    get_value(Kind::Governor)
}

/// Returns a vector of strings that represents the current frequency in kHz
/// the respective cpu is using, from cpu 0 to cpu X.
/// The vector is as large as the number of cpus.
// TODO use a more precise type than a String?
pub fn freq() -> Vec<String> {
    get_value(Kind::Freq)
}

/// Returns a vector of strings that represents the driver, or _policy_, the
/// respective cpu is using, from cpu 0 to cpu X.
/// The vector is as large as the number of cpus.
pub fn driver() -> Vec<String> {
    get_value(Kind::Driver)
}
