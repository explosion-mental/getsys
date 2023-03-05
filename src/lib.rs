//! `getsys` Library to get _some_ system _stuff_
/// The plan is to add some stable support (minor versions) and then release v2 (major version) that
/// implements proper error handling using thiserror macros.

/// Cpu functions
pub mod cpu;
pub use self::cpu as Cpu;

/// PerCpu functions
pub mod percpu;
pub use self::percpu as PerCpu;
