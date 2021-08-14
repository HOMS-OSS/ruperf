## Overview

See the [overview](https://HOMS-OSS.github.io/ruperf/docs/overview) for an introduction to the project.

## Requirements

Rust is required for this project. To download Rustup and install Rust:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

[More on installing Rust][3].

Linux 5.x+ is required to run this project.

## Build

To build this project:

- Checkout `main` branch from github using `git clone`.
- From top-level directory run `cargo build`. 
  - For best profiling specify `cargo build --release` as cargo will default to `debug`.

To build a sample program:
- ```bash
cargo build -p file-io
```

- ```bash
cargo build -p fp-calc
```

## Install

To install this project:

- With the project repository downloaded, run `cargo install --path *ruperf_path*` where `*ruperf_path*` is the path to the ruperf repository on your machine.
- For configuration information about `cargo install` including information about how to specify installation location, etc see [documentation][8]
- Another option is to build the code using `cargo build` and then `cd` into the `target` directory and select the target.

## Run

- See ```./ruperf --help```

- Examples:
  - ```bash
  ./ruperf stat -e cycles -e instructions -e task-clock -e L1D-cache-reads ls -a
  ```
  - ```bash
  ./ruperf test --json
  ```
  - ``` bash
  ./ruperf gui
  ```
- See our rustdocs for more documentation by running ```cargo doc --no-deps --open``` in the ruperf repository.

## Verification

Verification is done through a combination of `cargo test`, manual inspection comparing output of `perf stat` with output of `ruperf` on programs as documented in pull request history, and through inspection of contributor code.

## Demo/Walkthrough

Watch our demo video about `ruperf` on YouTube:

[![Ruperf Demo / Walkthrough](https://i.imgur.com/dFAPTuE.png)](https://youtu.be/tS1O9fe4wSM "Ruperf Demo / Walkthrough")

## Permissions

This tool uses the `perf_event_open()` system call, which requires some special permissions. 
While our tool currently checks if `perf_event_paranoid` is equal to 0,
this is less than ideal is some situations. A way around this is to change
the capabilities of the `ruperf` executable using [`setcap`](https://man7.org/linux/man-pages/man8/setcap.8.html). 

For Linux 5.8+, use `CAP_PERFMON`; otherwise use `CAP_SYSADMIN`.

## Contributing Guidelines

`ruperf` is an open source project and is open to recieving contributions!

Please see [`CONTRIBUTING`](https://github.com/HOMS-OSS/ruperf/blob/main/CONTRIBUTING.md)

## Code of Conduct

We are committed to providing a friendly, safe and welcoming environment for all, 
regardless of level of experience, gender identity and expression, 
sexual orientation, disability, personal appearance, 
body size, race, ethnicity, age, religion, nationality, or other similar characteristic.

[Rust Community Code of Conduct][4]

## References

[Linux-kernel wiki][5]

[The Rust Performance Book][6]

[Flame Graphs][7]

## License
[Gplv2][2]

### [Core Team](https://HOMS-OSS.github.io/ruperf/docs/team)





[1]:https://perf.wiki.kernel.org/index.php/Main_Page
[2]:https://github.com/HOMS-OSS/ruperf/blob/main/LICENSE
[3]:https://www.rust-lang.org/tools/install
[4]:https://www.rust-lang.org/policies/code-of-conduct
[5]:https://perf.wiki.kernel.org/index.php/Tutorial
[6]:https://nnethercote.github.io/perf-book/introduction.html
[7]:http://www.brendangregg.com/flamegraphs.html
[8]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
