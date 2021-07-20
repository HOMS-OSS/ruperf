//! Non-perf related system call wrappers.

pub fn read_wrap(fd: i32) -> isize {
    //read treats each counter as virtualized u64
    let mut count: isize = 0;
    unsafe {
        //buf must be *mut lbc::c_void type, mimics void pointer
        //package count into buf so it is easy to read
        let buf: *mut libc::c_void = &mut count as *mut _ as *mut libc::c_void;
        libc::read(fd, buf, std::mem::size_of_val(&count));
    }
    count
}
