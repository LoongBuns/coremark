mod list;
mod matrix;
mod state;

use list::LinkedList;
use matrix::Matrix;
use state::State;

#[link(wasm_import_module = "env")]
extern "C" {
    fn clock_ms() -> u32;
}

struct BenchmarkResult {
    name: &'static str,
    time_ms: f32,
    crc: u16,
}

#[export_name = "run"]
pub fn run() -> f32 {
    let mut total_time = 0.0;

    let list_result = run_benchmark("List Benchmark", benchmark_list);
    total_time += list_result.time_ms;

    let matrix_result = run_benchmark("Matrix Benchmark", benchmark_matrix);
    total_time += matrix_result.time_ms;

    let state_result = run_benchmark("State Machine Benchmark", benchmark_state);
    total_time += state_result.time_ms;

    println!("\nBenchmark Results:");
    println!("----------------------------");
    println!("{}: {:.2} ms, CRC: 0x{:04x}", list_result.name, list_result.time_ms, list_result.crc);
    println!("{}: {:.2} ms, CRC: 0x{:04x}", matrix_result.name, matrix_result.time_ms, matrix_result.crc);
    println!("{}: {:.2} ms, CRC: 0x{:04x}", state_result.name, state_result.time_ms, state_result.crc);
    println!("----------------------------");
    println!("Total Execution Time: {:.2} ms", total_time);

    total_time
}

fn run_benchmark<F>(name: &'static str, f: F) -> BenchmarkResult
where
    F: FnOnce() -> u16,
{
    unsafe {
        let start = clock_ms();
        let crc = f();
        let end = clock_ms();
        let time_ms = end.wrapping_sub(start) as f32;
        BenchmarkResult { name, time_ms, crc }
    }
}

fn benchmark_list() -> u16 {
    let mut crc = 0;

    let mut list = LinkedList::new();

    for i in 0..10_000 {
        list.push_front(i);
        crc = crc16(i, crc);
    }

    for _ in 0..10_000 {
        if let Some(val) = list.pop_front() {
            crc = crc16(val, crc);
        }
    }

    crc
}

fn benchmark_matrix() -> u16 {
    let size = 100;
    let mut crc = 0;

    let mut a = Matrix::new(size);
    let mut b = Matrix::new(size);

    for i in 0..size {
        for j in 0..size {
            a.set(i, j, i as i32 + j as i32);
            b.set(i, j, i as i32 * j as i32);
        }
    }

    let result = a.mul_matrix(&b);

    for i in 0..size {
        for j in 0..size {
            crc = crc16(result.get(i, j), crc);
        }
    }

    crc
}

fn benchmark_state() -> u16 {
    let mut crc = 0;
    let input = "1.23e+10,123,456.789,-123.45e-6,invalid";
    let results = State::transition_multiple(input, ',');

    for (token, state, _) in results {
        crc = crc16(state as i32, crc);
    }

    crc
}

fn crc16(data: i32, crc: u16) -> u16 {
    let mut crc = crc;
    let mut data = data as u16;

    for _ in 0..16 {
        if ((crc ^ data) & 0x1) != 0 {
            crc = (crc >> 1) ^ 0xA001;
        } else {
            crc >>= 1;
        }
        data >>= 1;
    }

    crc
}
