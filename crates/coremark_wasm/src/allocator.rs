/**
 * MIT License Copyright (c) 2022 Craig
 * 
 * https://github.com/Craig-Macomber/lol_alloc
 */

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
    ptr::{self, null_mut},
};

const PAGE_SIZE: usize = 65536;
const ERROR_PAGE_COUNT: PageCount = PageCount(usize::MAX);

#[derive(Eq, PartialEq)]
struct PageCount(usize);

impl PageCount {
    fn size_in_bytes(self) -> usize {
        self.0 * PAGE_SIZE
    }
}

trait MemoryGrower {
    fn memory_grow(&self, delta: PageCount) -> PageCount;
}

pub struct DefaultGrower;

#[cfg(target_arch = "wasm32")]
impl MemoryGrower for DefaultGrower {
    fn memory_grow(&self, delta: PageCount) -> PageCount {
        PageCount(core::arch::wasm32::memory_grow(0, delta.0))
    }
}

struct FreeListNode {
    next: *mut FreeListNode,
    size: usize,
}

const NODE_SIZE: usize = core::mem::size_of::<FreeListNode>();

pub struct FreeListAllocator<T = DefaultGrower> {
    free_list: UnsafeCell<*mut FreeListNode>,
    grower: T,
}

#[cfg(target_arch = "wasm32")]
impl FreeListAllocator<DefaultGrower> {
    pub const fn new() -> Self {
        FreeListAllocator {
            free_list: UnsafeCell::new(EMPTY_FREE_LIST),
            grower: DefaultGrower,
        }
    }
}

const EMPTY_FREE_LIST: *mut FreeListNode = usize::MAX as *mut FreeListNode;

unsafe impl<T> Send for FreeListAllocator<T> {}

unsafe impl<T: MemoryGrower> GlobalAlloc for FreeListAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        debug_assert!(PAGE_SIZE % layout.align() == 0);

        let size = full_size(layout);
        let alignment = layout.align().max(NODE_SIZE);
        let mut free_list: *mut *mut FreeListNode = self.free_list.get();
        loop {
            if *free_list == EMPTY_FREE_LIST {
                break;
            }
            let size_of_block = (**free_list).size;
            let start_of_block = *free_list as usize;
            let end_of_block = start_of_block + size_of_block;
            if size < end_of_block {
                let position = multiple_below(end_of_block - size, alignment);
                if position >= start_of_block {
                    let end_of_used = position + size;
                    if end_of_used < end_of_block {
                        let new_block = end_of_used as *mut FreeListNode;
                        (*new_block).next = *free_list;
                        (*new_block).size = end_of_block - end_of_used;
                        *free_list = new_block;
                        free_list = ptr::addr_of_mut!((*new_block).next);
                    }
                    if position == start_of_block {
                        *free_list = (**free_list).next;
                    } else {
                        (**free_list).size = position - start_of_block;
                    }

                    let ptr = position as *mut u8;
                    debug_assert!(ptr.align_offset(NODE_SIZE) == 0);
                    debug_assert!(ptr.align_offset(layout.align()) == 0);
                    return ptr;
                }
            }

            free_list = ptr::addr_of_mut!((**free_list).next);
        }

        let requested_bytes = round_up(size, PAGE_SIZE);
        let previous_page_count = self
            .grower
            .memory_grow(PageCount(requested_bytes / PAGE_SIZE));
        if previous_page_count == ERROR_PAGE_COUNT {
            return null_mut();
        }

        let ptr = previous_page_count.size_in_bytes() as *mut u8;
        self.dealloc(
            ptr,
            Layout::from_size_align_unchecked(requested_bytes, PAGE_SIZE),
        );
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug_assert!(ptr.align_offset(NODE_SIZE) == 0);
        let ptr = ptr as *mut FreeListNode;
        let size = full_size(layout);
        let after_new = offset_bytes(ptr, size);

        let mut free_list: *mut *mut FreeListNode = self.free_list.get();
        loop {
            if *free_list == EMPTY_FREE_LIST {
                (*ptr).next = EMPTY_FREE_LIST;
                (*ptr).size = size;
                *free_list = ptr;
                return;
            }

            if *free_list == after_new {
                let new_size = size + (**free_list).size;
                let next = (**free_list).next;
                if next != EMPTY_FREE_LIST && offset_bytes(next, (*next).size) == ptr {
                    (*next).size += new_size;
                    *free_list = next;
                    return;
                }
                *free_list = ptr;
                (*ptr).size = new_size;
                (*ptr).next = next;
                return;
            }

            if *free_list < ptr {
                if offset_bytes(*free_list, (**free_list).size) == ptr {
                    (**free_list).size += size;
                    return;
                }
                (*ptr).next = *free_list;
                (*ptr).size = size;
                *free_list = ptr;
                return;
            }
            free_list = ptr::addr_of_mut!((**free_list).next);
        }
    }
}

fn full_size(layout: Layout) -> usize {
    let grown = layout.size().max(NODE_SIZE);
    round_up(grown, NODE_SIZE)
}

fn round_up(value: usize, increment: usize) -> usize {
    debug_assert!(increment.is_power_of_two());

    multiple_below(value + (increment - 1), increment)
}

fn multiple_below(value: usize, increment: usize) -> usize {
    debug_assert!(increment.is_power_of_two());

    value & increment.wrapping_neg()
}

unsafe fn offset_bytes(ptr: *mut FreeListNode, offset: usize) -> *mut FreeListNode {
    (ptr as *mut u8).add(offset) as *mut FreeListNode
}