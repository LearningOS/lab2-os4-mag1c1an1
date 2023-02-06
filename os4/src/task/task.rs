use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    mm::{MapPermission, MemorySet, PhysPageNum, VirtAddr, KERNEL_SPACE},
    syscall::TaskInfo,
    trap::{trap_handler, TrapContext},
};

use super::TaskContext;

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    #[allow(unused)]
    UnInit,
    Ready,
    Running,
    Exited,
}
pub struct TaskControlBlock {
    // pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub info: TaskInfo,
    /// start time
    pub stime: usize,
}

impl TaskControlBlock {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let info = TaskInfo::init();
        let (kstack_bottom, kstack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.lock().insert_framed_area(
            kstack_bottom.into(),
            kstack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let tcb = Self {
            info,
            task_cx: TaskContext::goto_trap_return(kstack_top),
            memory_set,
            trap_cx_ppn,
            base_size: user_sp,
            stime: 0,
        };
        let trap_cx = tcb.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.lock().token(),
            kstack_top,
            trap_handler as usize,
        );
        tcb
    }
}
