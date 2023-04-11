#![doc = include_str!("../README.md")]
//! \n
//! # Thoughts (these are not TODO's)
//! * release v2 (major version) that implements proper error handling using thiserror macros
//!   (This is still an idea, given that this lib **only** reads /sys fs, which should exist in order
//!   for this to work)
//! * use `num_cpus` crate, instead of [`glob`]ing the results. This removes searching for the
//!   paths, and just directly write to it. Can also remove errors from a glob pattern, although it
//!   should not be a problem given how simple it is (`../cpu[0-9]*/..`)
//! * Consider using features for Cpu or PerCpu, rather than including everything

/// Cpu functions
pub mod cpu;
pub use self::cpu as Cpu;

/// PerCpu functions
pub mod percpu;
pub use self::percpu as PerCpu;
