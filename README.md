# Ruperf

Timothy Maloney, Briana Oursler, Greg Hairfield, Sam Little, Michael Scherrer 2021

Ruperf is a Rust adaptation of the [linux perf command][1], also called perf_events. Ruperf provides minimal support for the `perf stat` and `perf test` commands and is run by either its `gui` application or its command line interface driver.


Minimal support to proof of concept these high-level commands.
- `perf-test`: runs assorted sanity tests
- `perf stat`: gathers performance counter statistics

*Perf stat coverage* (MVP)

Hardware events
- cpu-cycles OR cycles
- instructions 
- L1-dcache-loads

Software events
- task-clock
- context-switches

*Perf test coverage*
- check for libpfm4
- check status of `perf_event_paranoid`
- examples of how to extend test suite
- sanity check of ruperf timing events


The Future
*Extend support*
- Continue to build on `test` and `stat` coverage. Add support for `record` and `report`. `Report` gathers
  statistics about performance and returns interesting tables or graphs.
- Add graphs to capture interesting information from event counters.
- Support Windows and Mac platforms.
- Custom support for Rust program profiling.


## Requirements

Rust is required for this project.

It is recommended to download Rustup and install Rust.

In terminal type:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

More information about installing Rust [here][3].

Linux 5.x+ is required to run this project.

## Build

To build this project:

- Checkout `main` branch from github using `git clone`.
- From top-level directory run `cargo build`. For best profiling specify `cargo build --release` as cargo will default to `debug`.

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
## Contributing Guidelines

Ruperf is an open source project and is open to recieving contributions!

Please see [`CONTRIBUTING.md`](https://github.com/HOMS-OSS/ruperf/blob/main/CONTRIBUTING.md)

## Code of Conduct

 We are committed to providing a friendly, safe and welcoming environment for all, regardless of level of experience, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, nationality, or other similar characteristic.

 More from [Rust Community Code of Conduct][4]

## Our Contributors

- Briana Oursler. My contributions to the project principally relate to the `stat` functionality performed by ruperf, including support for `read`, the `event struct`, and additions to hardware and software event counters. I implemented initial support for the CLI tool using the `structopt` crate. I worked along with Timothy Maloney on the team to set up `perf_event_open`. I collaborated with Timothy Maloney and Michael Scherrer on using `bindgen` for C code we needed bindings for that we couldn't access through `libc`, principally related to `perf_event`, and on establishing support for running a program to profile.

## Further Reading

See `whitepaper.tex`

## References

[Linux-kernel wiki][5]

[The Rust Performance Book][6]

[Flame Graphs][7]

## License
[Gplv2][2]








[1]:https://perf.wiki.kernel.org/index.php/Main_Page
[2]:https://github.com/HOMS-OSS/ruperf/blob/main/LICENSE
[3]:https://www.rust-lang.org/tools/install
[4]:https://www.rust-lang.org/policies/code-of-conduct
[5]:https://perf.wiki.kernel.org/index.php/Tutorial
[6]:https://nnethercote.github.io/perf-book/introduction.html
[7]:http://www.brendangregg.com/flamegraphs.html
[8]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
