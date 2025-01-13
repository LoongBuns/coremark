#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate alloc;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod allocator;

use core::alloc::{GlobalAlloc, Layout};

#[cfg(target_arch = "wasm32")]
use allocator::FreeListAllocator;

pub struct SyncAllocator<T>(T);

unsafe impl<T> Sync for SyncAllocator<T> {}

unsafe impl<T: GlobalAlloc> GlobalAlloc for SyncAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout);
    }
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: SyncAllocator<FreeListAllocator> = SyncAllocator(FreeListAllocator::new());

mod list;
mod matrix;
mod state;

use list::LinkedList;
use matrix::Matrix;
use state::State;

#[link(wasm_import_module = "env")]
extern "C" {
    fn clock_ms() -> i64;
}

#[export_name = "run"]
pub fn run() -> f32 {
    let mut total_time = 0;

    unsafe {
        total_time += benchmark_list();
        total_time += benchmark_matrix();
        total_time += benchmark_state();
    }

    total_time as f32 / 1000.0
}

unsafe fn benchmark_list() -> i64 {
    let start_time = clock_ms();

    let mut list = LinkedList::<i16>::new();
    let size = 10;

    for i in 0..size {
        list.push_front(i);
    }

    list.reverse();
    list.find(|&x| x == size / 2);

    for _ in 0..size {
        list.pop_front();
    }

    let end_time = clock_ms();
    end_time - start_time
}

unsafe fn benchmark_matrix() -> i64 {
    let start_time = clock_ms();

    let size = 3;
    let mut matrix_a = Matrix::<i16>::new(size);
    let mut matrix_b = Matrix::<i16>::new(size);

    for i in 0..size {
        for j in 0..size {
            matrix_a.set(i, j, (i + j) as i16);
            matrix_b.set(i, j, (i - j) as i16);
        }
    }

    matrix_a.mul_const(2);
    matrix_b.add_const(-2);
    matrix_a.mul_vect(&vec![1, 2, 3]);
    matrix_a.mul_matrix(&matrix_b);

    let end_time = clock_ms();
    end_time - start_time
}

unsafe fn benchmark_state() -> i64 {
    let start_time = clock_ms();

    for token in ["123.45e-6", "678", "invalid", "42e2", ".5"] {
        State::transition(token.as_bytes());
    }

    let end_time = clock_ms();
    end_time - start_time
}
