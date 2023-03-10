//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::va2pa;
use crate::task::{
    exit_current_and_run_next, get_task_info, suspend_current_and_run_next, TaskStatus, mmap, munmap,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}
impl TaskInfo {
    pub fn init() -> TaskInfo {
        Self {
            status: TaskStatus::Ready,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let pa = va2pa((ts as usize).into());
    if let Some(pa) = pa {
        let us = get_time_us();
        let ts = pa.0 as *mut TimeVal;
        unsafe {
            *ts = TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            };
        }
        0
    } else {
        -1
    }
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    mmap(start,len,port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    munmap(start,len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("sys_task_info");
    if let Some(pa) = va2pa((ti as usize).into()) {
        let ti = pa.0 as *mut TaskInfo;
        unsafe { *ti = get_task_info() }
        0
    } else {
        error!("sys_task_info failed!");
        -1
    }
}
