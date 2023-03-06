#![doc = include_str!("../README.md")]
//! \n
//! # Thoughts
//! * [X] add some stable support (minor versions)
//! * [ ] release v2 (major version) that implements proper error handling using thiserror macros
//! (This is still an idea, given that this lib **only** reads /sys fs, which should exist in order
//! for this to work)

/// Cpu functions
pub mod cpu;
pub use self::cpu as Cpu;

/// PerCpu functions
pub mod percpu;
pub use self::percpu as PerCpu;
