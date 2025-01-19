#![cfg_attr(all(not(test), target_arch = "wasm32"), no_std)]

#[macro_use]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

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

mod allocator;

#[cfg(all(not(test), target_arch = "wasm32"))]
use allocator::FreeListAllocator;

#[cfg(all(not(test), target_arch = "wasm32"))]
#[global_allocator]
static ALLOCATOR: SyncAllocator<FreeListAllocator> = SyncAllocator(FreeListAllocator::new());

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

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

static mut CRC: u16 = 0;

#[export_name = "run"]
pub fn run() -> f32 {
    let mut timer_ms: i64;

    unsafe {
        timer_ms = clock_ms();

        let iterations = 100_000;
        let mut crc: u16 = 0;

        for _ in 0..iterations {
            benchmark_list(&mut crc);
            benchmark_matrix(&mut crc);
            benchmark_state(&mut crc);
        }
        timer_ms = clock_ms() - timer_ms;

        CRC = crc16(&crc.to_le_bytes(), CRC);
    }

    timer_ms as f32
}

fn benchmark_list(crc: &mut u16) {
    let mut list = LinkedList::<i16>::new();
    let size = 10;

    for i in 0..size {
        list.push_front(i);
    }

    *crc = crc16(&(list.size() as u32).to_le_bytes(), *crc);

    list.reverse();

    if let Some(found) = list.find(|&x| x == (size / 2) as i16) {
        *crc = crc16(&found.to_le_bytes(), *crc);
    }

    list.mergesort(&|a, b| a.cmp(b));

    list.insert_after(&(size / 2 as i16), size * 2);

    if let Some(removed_val) = list.remove_after(&(size * 2)) {
        *crc = crc16(&removed_val.to_le_bytes(), *crc);
    }

    while let Some(v) = list.pop_front() {
        if v == 0 {
            *crc = crc16(&v.to_le_bytes(), *crc);
        }
    }
}

fn benchmark_matrix(crc: &mut u16) {
    let size = 3;
    let mut matrix_a = Matrix::<i16>::new(size, size);
    let mut matrix_b = Matrix::<i16>::new(size, size);

    for i in 0..size {
        for j in 0..size {
            matrix_a.set(i, j, (i + j) as i16);
            matrix_b.set(i, j, (i - j) as i16);
        }
    }

    let mid_val = matrix_a.get(1, 1);
    *crc = crc16(&mid_val.to_le_bytes(), *crc);

    matrix_a.mul_const(2);
    matrix_b.add_const(-2);

    let inv_a = matrix_a.inverse();
    if let Some(inv) = inv_a {
        let inv_val = inv.get(1, 1);
        *crc = crc16(&inv_val.to_le_bytes(), *crc);
    }

    let vect_res = matrix_a.mul_vect(&[1, 2, 3]);
    *crc = crc16(&vect_res[1].to_le_bytes(), *crc);

    let mat_c = matrix_a.mul_matrix(&matrix_b);

    let mut sum: i16 = 0;
    for i in 0..size {
        for j in 0..size {
            sum = sum.wrapping_add(mat_c.get(i, j));
        }
    }
    *crc = crc16(&sum.to_le_bytes(), *crc);
}

fn benchmark_state(crc: &mut u16) {
    for token in ["123.45e-6", "678", "invalid", "42e2", ".5"] {
        let (final_state, path) = State::transition(token.as_bytes());
        *crc = crc16(&[(final_state as u8)], *crc);
        let path_len = path.len() as u32;
        *crc = crc16(&path_len.to_le_bytes(), *crc);
    }
}

fn crc16(data: &[u8], mut crc: u16) -> u16 {
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
