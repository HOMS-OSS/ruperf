/ * 
	license
* /

//! Prototype wrapper the `perf_event_open()` system-call.
//!
//! No wrapper is provided for `perf_event_open()`;
//! necessitating use of `syscall()`.
//! see Linux man-page NOTES for `perf_event_open()` for details.

#ifndef OPEN_H
#define OPEN_H

#include <linux/perf_event.h>
#include <linux/hw_breakpoint.h>
#include <sys/syscall.h>
#include <unistd.h>

static long 
perf_event_open(struct perf_event_attr *hw_event,
		pid_t pid,
		int cpu,
		int group_fd,
		unsigned long flags);
#endif
