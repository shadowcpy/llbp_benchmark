use std::time::Duration;

use llbp_benchmark::m5_fail;
use rand::RngCore;

pub const ARRAY_SIZES: &[usize] = &[256];
pub const MIN_TIME: Duration = Duration::from_secs(10);

fn main() {
    divan::main();
}

#[divan::bench(args = ARRAY_SIZES, min_time = MIN_TIME)]
fn binary_search(bencher: divan::Bencher, n: usize) {
    m5_fail();
    bencher
        .with_inputs(|| {
            let mut rng = rand::rng();
            let mut target = vec![0; n];
            for v in target.iter_mut() {
                *v = rng.next_u64() % 256;
            }
            target.sort_unstable();
            (target, rng.next_u64() % 256)
        })
        .bench_local_values(|(v, r)| do_binary_search(divan::black_box(v), divan::black_box(r)));
}

fn do_binary_search(v: Vec<u64>, r: u64) -> Option<usize> {
    // Manual binary search
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if v[mid] == r {
            return Some(mid);
        } else if r > v[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}
