/ *
	license
* /

//! Definiton of wrapper to `perf_event_open()` system call. 
//!
//! `perf_event_open` returns a file descriptor that allows 
//! measuring performance information. Each file descriptor 
//! corresponds to one event that is being measured. The file
//! descriptor is then used in subsequent system calls.
//! 
//! Events can be enabled/disabled by using either of:
//!
//! * `ioctl()`
//! * `prctl()`
//!
//! Disabled events maintain their existence and count values 
//! but they are not counted, nor do they generate overflows.
//!
//! Two types of events: counting and sampled. Counting events
//! count the aggregate number of events that occur; and can
//! gathered by calling `read()`. Sampling events periodically 
//! write measurements to a buffer that can be accessed by 
//! calling `mmap()`.

#include "open.h"

static long 
perf_event_open(struct perf_event_attr *event,
		pid_t pid,
		int cpu,
		int group_fd,
		unsigned long flags)
{
	int ret;
	ret = syscall(__NR_perf_event_open,
			event,
			cpu,
			group_fd,
			flags);
	return ret;
}
