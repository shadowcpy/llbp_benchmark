use std::{hint::black_box, time::Duration};

use llbp_benchmark::{random_vec, try_cmd};
use rand::{RngCore, rng};

pub const ARRAY_SIZES: &[usize] = &[256];
pub const MIN_TIME: Duration = Duration::from_secs(10);

#[divan::bench(args = ARRAY_SIZES, min_time = MIN_TIME)]
fn naive_binary_search(bencher: divan::Bencher, n: usize) {
    try_cmd("/usr/local/bin/m5", ["fail", "4"]);
    bencher
        .with_inputs(|| {
            let mut target = random_vec(n);
            let random_val = rng().next_u64() % target.len() as u64;
            target.sort_unstable();
            (target, random_val)
        })
        .bench_local_values(|(arr, val)| do_naive_binary_search(black_box(arr), black_box(val)));
}

/// Manual, naive binary search
fn do_naive_binary_search(arr: Vec<u64>, val: u64) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == val {
            return Some(mid);
        } else if val > arr[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    divan::main();
}
