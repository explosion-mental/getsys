//! CPU related functions
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Cpu {}

impl Cpu {
    ///Returns true if the turbo boost is enabled, false if it is disabled or not supported.
    pub fn turbo() -> bool {
        let path;
        let intelpstate = "/sys/devices/system/cpu/intel_pstate/no_turbo";
        let cpufreq = "/sys/devices/system/cpu/cpufreq/boost";

        if Path::new(intelpstate).exists() {
            path = intelpstate;
        } else if Path::new(cpufreq).exists() {
            path = cpufreq;
        } else {
            return false;
        }


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

