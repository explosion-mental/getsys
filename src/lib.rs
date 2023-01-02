//! Library to get _some_ system _stuff_
///The plan is to add some stable support (minor versions) and then release v2 (major version) that
///implements proper error handling using thiserror macros.

mod util;
use util::{get_turbo_path, TurboBoost};
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use glob::glob;

/// CPU related functions
pub struct Cpu {}

impl Cpu {
    ///Returns true if the turbo boost is enabled, false if it is disabled or not supported.
    pub fn turbo() -> bool {
        let path = match get_turbo_path() {
            TurboBoost::None => return false,
            TurboBoost::Intelp => "/sys/devices/system/cpu/intel_pstate/no_turbo",
            TurboBoost::CpuFreq => "/sys/devices/system/cpu/cpufreq/boost",
        };

        if fs::read_to_string(path).expect("sysfs file shoudln't return an error").trim() == "1" {
            return true
        }

        false
    }

    ///Average CPU usage as a f64 value percentage from 0% - 100%. It takes as a parameter the
    ///amount of time to sleep in between reads, since to get an average of the usage, we will need
    ///to compare values passed an interval of time.
    pub fn perc(sleeptime: std::time::Duration) -> f64 {

        /* read the first line of /proc/stat */
        let read = || {
            let mut firstline = String::new();
            let mut buffer = std::io::BufReader::new(
                File::open("/proc/stat").expect("Unable to open '/proc/stat'.")
                );
            buffer.read_line(&mut firstline).expect("Unable to read '/proc/stat'.");
            firstline
        };

        let s = read();
        let mut s = s.split_ascii_whitespace();

        /* cpu user nice system idle iowait irq softirq
         *        0    1      2    3      4   5       6
         */
        s.next().unwrap(); //ignore the "cpu" word
        let user  = s.next().unwrap().parse::<f64>().unwrap();
        let nice  = s.next().unwrap().parse::<f64>().unwrap();
        let sys   = s.next().unwrap().parse::<f64>().unwrap();
        let idle  = s.next().unwrap().parse::<f64>().unwrap();
        let wait  = s.next().unwrap().parse::<f64>().unwrap();
        let irq   = s.next().unwrap().parse::<f64>().unwrap();
        let sirq  = s.next().unwrap().parse::<f64>().unwrap();

        /* sleep */
        std::thread::sleep(sleeptime);

        //agane..
        let s = read();
        let mut s = s.split_ascii_whitespace();

        s.next().unwrap(); //ignore the "cpu" word
        let user2 = s.next().unwrap().parse::<f64>().unwrap();
        let nice2 = s.next().unwrap().parse::<f64>().unwrap();
        let  sys2 = s.next().unwrap().parse::<f64>().unwrap();
        let idle2 = s.next().unwrap().parse::<f64>().unwrap();
        let wait2 = s.next().unwrap().parse::<f64>().unwrap();
        let  irq2 = s.next().unwrap().parse::<f64>().unwrap();
        let sirq2 = s.next().unwrap().parse::<f64>().unwrap();

        /* get an average */

        if user2 == 0.0 { return 0.0 }

        let sum = (user2 + nice2 + sys2 + idle2 + wait2 + irq2 + sirq2) -
                  (user  + nice  + sys  + idle  + wait  + irq  + sirq );

        if sum == 0.0 { return 0.0 }

        100.0 * ((user2 + nice2 + sys2 + irq2 + sirq2) -
                 (user  + nice  + sys  + irq  + sirq)) / sum
    }

    ///Average system temperature
    pub fn temp() -> u32 {
        use std::path::Path;

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
}

/// Per cpu information, rather than average or aggregates like Cpu
pub struct PerCpu {
}

//TODO there is more per cpu information in /proc/stat/
///Reference: https://www.kernel.org/doc/html/v4.14/admin-guide/pm/cpufreq.html
impl PerCpu {
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
}
