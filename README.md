# getsys - get _some_ system _stuff_

Support: Linux.

Mainly made to have a simpler and straightforward interface, and backend.

# Features
- cpu turbo boost state
- average temperature
- cpu percentage over an interval of `Duration`
- Per cpu values:
	* [scaling govenor](https://www.kernel.org/doc/html/v4.14/admin-guide/pm/cpufreq.html)
	* driver
	* frequency

# docs
```
cargo doc --open
```

# crates

- glob 0.3.0

# Honorable Mentions

I have written this little library to fetch information I couldn't get with other crates, but there are very nicely writen crates that provide information about the system that I need.

- battery - [crates.io](https://crates.io/crates/battery) - [github](https://github.com/svartalf/rust-battery)
- num_cpus - [crates.io](https://crates.io/crates/num_cpus) - [github](https://github.com/seanmonstar/num_cpus)
