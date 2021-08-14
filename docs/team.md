# Core Team

## Briana Oursler

My contributions to the project principally relate to the `stat` functionality performed by ruperf, 
including support for `read`, the `event struct`, and additions to hardware and software event counters. 
I implemented initial support for the CLI tool using the `structopt` crate. 
I worked along with Timothy Maloney on the team to set up `perf_event_open`. I collaborated with Timothy Maloney
and Michael Scherrer on using `bindgen` for C code we needed bindings for that 
we couldn't access through `libc`, principally related to `perf_event`, and on establishing 
support for running a program to profile.


## Timothy Maloney

My main contributions have been overseeing the documentation and research necessary for our project;
and collaborating with Briana to create a safe and efficient interface 
for interacting with Linux's system calls and setting up performance event monitoring.
I also added support for the `ioctl()` system call,
and support for accessing cache information. I worked with Briana and Michael to add program profiling
to `ruperf stat`; and am the creator and maintaner of our [website](https://HOMS-OSS.github.io/ruperf/). 


## Michael Scherrer

My Contributions include developing and extensible graphical user interface to run commands such as `ruperf test` and `ruperf stat` in a clean, easy to use environment, it includes loading and saving of previously ran commands, so that they may be used again at any time. In addition I initialized rust C bindings for the rust compiler. 



## Sam Little

My contributions to this project were mostly contained in the `test`
subcommand, which runs a collection of various environment checks and sanity tests
required for `ruperf`. I tried to keep the structure of it as similar as possible
to the real `perf test`, which is used for the same purpose. The output of `test`
looks visually similar to `perf test`, tests can be skipped (and only some can be
run if specified), and it can also be `>`-ed as JSON to a file with a `--json`
flag for ease-of-programmatic-use. I also wrote some tests that relate to `ruperf`'s
MVP, and when designing `test` tried to keep extensibility in mind so that it would
be easy to add more complex tests later.


## Greg Hairfield




