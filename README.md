# getsys - get _some_ system _stuff_
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/getsys)
[![license](https://img.shields.io/badge/license-MIT-lightgreen?style=flat-square)](./LICENSE)
[![loc](https://img.shields.io/tokei/lines/github/explosion-mental/racf?color=lightgreen&style=flat-square)](./src)
[![cratesv](https://img.shields.io/crates/v/getsys?style=flat-square&color=red)](https://crates.io/crates/getsys)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/getsys/status.svg)](https://deps.rs/repo/codeberg/explosion-mental/getsys)
<br>
Support: Linux.

Mainly made to have a simpler and straightforward interface, and backend.

## Features

### Cpu
- cpu turbo boost state
- average temperature
- cpu percentage over an interval of `Duration`
### Per Cpu
- [scaling govenor](https://www.kernel.org/doc/html/v4.14/admin-guide/pm/cpufreq.html)
- driver
- frequency

## crates

- glob 0.3.0

## Honorable Mentions

I have written this little library to fetch information I couldn't get with other crates, but there are very nicely writen crates that provide information about the system that this crate doesn't provide:

- battery - [crates.io](https://crates.io/crates/battery) - [github](https://github.com/svartalf/rust-battery)
- num_cpus - [crates.io](https://crates.io/crates/num_cpus) - [github](https://github.com/seanmonstar/num_cpus)
