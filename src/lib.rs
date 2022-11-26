mod utils;
use utils::{get_turbo_path, read_path};
use utils::TurboBoost;
use std::io::prelude::*;
use std::fs::File;
use glob::glob;

/// Cpu interface
pub struct Cpu {}

impl Cpu {
    ///Returns true if the turbo boost is enabled, false if it is disabled or not supported.
    pub fn turbo() -> bool {
        let path;

        match get_turbo_path() {
            TurboBoost::None => return false,
            TurboBoost::Intelp => path = "/sys/devices/system/cpu/intel_pstate/no_turbo",
            TurboBoost::CpuFreq => path = "/sys/devices/system/cpu/cpufreq/boost",
        }

        if read_path(path).trim() == "1" {
            return true
        } else {
            return false
        }
    }

    ///Returns the number of cores of the system, basically nproc().
    pub fn cores() -> u32 {
        //TODO use sysconf
        let mut cnt: u32 = 0;
        for _i in glob("/sys/devices/system/cpu/cpu[0-9]*").expect("Failed to read glob pattern") {
            cnt += 1;
        }
        cnt
    }

    ///Average CPU usage as a f64 value percentage from 0% - 100%. It takes as a parameter the
    ///amount of time to sleep in between reads, since to get an average of the usage, we will need
    ///to compare values passed an interval of time.
    pub fn perc(time: u32) -> f64 {

        if time == 0 { return 0.0 } //result will be 0 anyway.

        /* read the first line of /proc/stat */
        let mut firstline = String::new();
        let mut buffer = std::io::BufReader::new(
                    File::open("/proc/stat").unwrap()
                    );
        buffer.read_line(&mut firstline).expect("Unable to read line");

        let mut s = firstline.split_ascii_whitespace();

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
        std::thread::sleep(std::time::Duration::from_secs(time.into()));

        //agane..
        let mut firstline = String::new();
        let mut buffer = std::io::BufReader::new(
                    File::open("/proc/stat").unwrap()
                    );
        buffer.read_line(&mut firstline).expect("Unable to read line");
        let mut s = firstline.split_ascii_whitespace();

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
                  (user + nice + sys + idle + wait + irq + sirq);

        if sum == 0.0 { return 0.0 }

        100.0 * ((user2 + nice2 + sys2 + irq2 + sirq2) -
                 (user  + nice  + sys  + irq  + sirq)) / sum
    }

    ///Average system temperature
    pub fn temp() -> u32 {
        //TODO check for paths
        let path = "/sys/class/thermal/thermal_zone0/temp";

        read_path(path).trim().parse::<u32>().unwrap() / 1000 as u32
    }
}

