<p style="text-align:center;"><img src="./assets/Ruperf-The-Crab.jpg" width="500" height="333" class="center"></p>
## Introduction

All software should perform at a high level. That means going underneath the hood, 
locating areas of improvement, and fine tuning the machine. 
Hardware instructions, computer cycles, cache-accesses,
and context switches; how much memory is being used or how long a process takes; 
these and more, are all things to observe when seeking insight into where and how to improve performance.

`ruperf` was borne out of a desire to achieve three things:

- Provide a well-documented, high-level, performance analysis tool that adds to the existing landscape 
  by improving and expanding upon what current tools offer.

- Leverage the safety of Rust and and it's module system for a secure and fine-grained approach 
  to performance analysis.

- Explore Rust in the Linux Kernel. We would like to eventually lower the scope in which Rust is used
  to beneath the system call level, and experiment with it's safety and performance features there.


## Existing landscape

The current landscape of analyzing software performance has many tools. 
They all have different features, and work at various levels of the machine.
These are some of the tools that inspired our project:

#### `perf`

`perf` is a profiler tool for linux-based systems that abstracts away CPU hardware differences in 
Linux performance measurements and presents a simple command-line interface. 

#### VTune

VTune is a closed-source performance analysis tool provided by Intel. It identifies
time-intensive functions, time spent on I/O, cache misses, branch mispredictions, and more.

#### Gprof

Gprof is for Unix applications, and uses instrumentation and sampling to 
track code execution time and locate program hot spots. 

#### DHAT: dynamic heap analysis tool

Part of Valgrind, DHAT examines program use of heap allocation. 
It tracks allocated blocks, and inspects memory access. 
It also identifies process-lifetime leaks, excessive turnover, 
short-lived blocks, unused or underused allocations, and inefficiently laid-out blocks.

#### flamegraph

FlameGraph is a tool that provides a visual representation of profiled software.
which makes it easier to understand where a program is spending it's time.


## Overview of `ruperf`

Each of these tools are powerful in their own context. Utilizing them together 
can provide even greater leverage for calibrating software. By looking to the past
and identifying which features are essential, which are unnecessary, and which can be added,
we can more effectively address performance problems in the modern era of software.

While `ruperf` is still in the beginning stages of development, we believe a solid
and innovative foundation is there. `stat` and `test` currently mimic their `perf` counterparts,
and our goals for expanding upon them further involve adding timer support for events.
`gui` is an extremely helpful sub-command that combines the aid of data-visualization with the ease of a CLI.
While these are the minimum features we've described in our MVP, we are planning to add more.

Our goals for lowering the scope of Rust beneath the system call level are so that
we may further explore it's potential as a safe and high-performing systems language.
As you can probably guess, some of us really love Rust, and there isn't much
performance-analysis related Rust stuff in the current scope of things. Our hope is that
this project highlights the pros of using Rust in a context as sensitive as 
hardware performance counter information.


## What's in Development

Want to contribute? Interested in what we're working on? Here are things we need done!

#### For how to contribute, [see `CONTRIBUTING`](https://github.com/HOMS-OSS/ruperf/blob/main/CONTRIBUTING.md).


- A safe wrapper for the `mmap()` Linux system call.
- `ruperf record`.
- `ruperf report`.
- Adding support for timers.
- New programs to profile.
- Lowering the scope of Rust beneath the system call level.
