mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
// use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
// pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
pub use address::VPNRange;
use page_table::PageTable;

use crate::task::current_user_token;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub fn va2pa(va: VirtAddr) -> Option<PhysAddr> {
    let vpn = va.floor();
    let ppn = PageTable::from_token(current_user_token())
        .translate(vpn)
        .map(|ent| ent.ppn());
    if let Some(ppn) = ppn {
        let mut pa: PhysAddr = ppn.into();
        pa = (pa.0+va.page_offset()).into();
        Some(pa)
    }else {
        error!("va2pa failed!");
        None
    }
}
