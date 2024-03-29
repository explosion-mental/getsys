# getsys - get _some_ system _stuff_
[![crates io](https://img.shields.io/crates/v/getsys?style=flat-square&color=red)](https://crates.io/crates/getsys)
[![lib rs](https://img.shields.io/badge/lib.rs-latest-84f)](https://lib.rs/getsys)
[![downloads](https://img.shields.io/crates/d/getsys?style=flat-square&color=yellow)](https://crates.io/crates/getsys)
[![license](https://img.shields.io/crates/l/getsys?style=flat-square)](https://codeberg.org/explosion-mental/getsys/src/branch/main/LICENSE)
[![dependency status](https://deps.rs/repo/codeberg/explosion-mental/getsys/status.svg?style=flat-square&color=purple)](https://deps.rs/repo/codeberg/explosion-mental/getsys)
[![loc](https://img.shields.io/tokei/lines/github/explosion-mental/getsys?color=lightgreen&style=flat-square)](https://codeberg.org/explosion-mental/getsys/src/branch/main/src)
[![CodeBerg](https://img.shields.io/badge/Hosted_at-Codeberg-%232185D0?style=flat-square&logo=CodeBerg)](https://codeberg.org/explosion-mental/getsys)
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
