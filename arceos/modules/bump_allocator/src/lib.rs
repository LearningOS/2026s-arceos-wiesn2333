#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start_vaddr: usize,
    size: usize,
    b_pos: usize,
    p_pos: usize,
    count: usize,
    pages_allocated: usize,
}

const fn align_down(pos: usize, align: usize) -> usize {
    pos & !(align - 1)
}
const fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start_vaddr: 0,
            size: 0,
            b_pos: 0,
            p_pos: 0,
            count: 0,
            pages_allocated: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start_vaddr = start;
        self.size = size;
        self.b_pos = start;
        self.p_pos = start + size;
        self.count = 0;
        self.pages_allocated = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        todo!()
    }
}

struct Node {
    size: usize,
    align: usize,
    prev: usize,
    next: usize,
}
const NODE_SIZE: usize = core::mem::size_of::<Node>();

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let node_addr = align_up(self.b_pos, layout.align());
        let addr = align_up(node_addr + NODE_SIZE, layout.align());
        if addr + layout.size() > self.p_pos {
            return allocator::AllocResult::Err(allocator::AllocError::NoMemory);
        }
        let node = unsafe { &mut *(node_addr as *mut Node) };
        node.size = layout.size();
        node.align = layout.align();
        node.prev = self.b_pos;
        node.next = 0;

        self.b_pos = addr + layout.size() + NODE_SIZE;
        self.count += 1;
        allocator::AllocResult::Ok(core::ptr::NonNull::new(addr as *mut u8).unwrap())
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        let node_addr = align_down(pos.as_ptr() as usize - NODE_SIZE, layout.align());
        let node = unsafe { &mut *(node_addr as *mut Node) };
        if node.size != layout.size() {
            panic!("dealloc size mismatch");
        }
        if node.next == 0 {
            let prev_node = unsafe { &mut *(node.prev as *mut Node) };
            prev_node.next = 0;
            self.b_pos = align_up(node.prev + NODE_SIZE, node.align) + node.size;
        }else{
            let next_node = unsafe { &mut *(node.next as *mut Node) };
            next_node.prev = node.prev;
            if node_addr != align_up(self.start_vaddr, node.align) {
                let prev_node = unsafe { &mut *(node.prev as *mut Node) };
                prev_node.next = node.next;
            }
            self.b_pos = align_up(node_addr + NODE_SIZE, node.align) + node.size;
        }
        self.count -= 1;
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        todo!()
    }

    fn available_bytes(&self) -> usize {
        todo!()
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        todo!()
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}
