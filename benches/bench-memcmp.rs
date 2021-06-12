use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::cmp::Ordering;

mod barrier;
use barrier::memory_barrier;

#[inline(never)]
fn process_std_memcmp(texts: &[&str], pattern: &str) -> (usize, usize, usize) {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found_eq: usize = 0;
    let mut found_le: usize = 0;
    let mut found_gr: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let r = (&line_bytes[i..(i + pat_len)]).cmp(&pat_bytes);
            match r {
                Ordering::Equal => found_eq += 1,
                Ordering::Less => found_le += 1,
                Ordering::Greater => found_gr += 1,
            }
        }
    }
    (found_eq, found_le, found_gr)
}

#[inline(never)]
fn process_libc_memcmp(texts: &[&str], pattern: &str) -> (usize, usize, usize) {
    // original libc function
    extern {
        fn memcmp(cx: *const u8, ct: *const u8, n: usize) -> i32;
    }
    #[inline(always)]
    fn _x_libc_memcmp(a: &[u8], b: &[u8]) -> Ordering {
        let cx = a.as_ptr();
        let ct = b.as_ptr();
        let a_len = a.len();
        let b_len = b.len();
        if a_len != b_len {
            if a_len < b_len {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        let r = unsafe { memcmp(cx, ct, a_len) };
        if r == 0 {
            Ordering::Equal
        } else if r < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found_eq: usize = 0;
    let mut found_le: usize = 0;
    let mut found_gr: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let r = _x_libc_memcmp(&line_bytes[i..(i + pat_len)], pat_bytes);
            match r {
                Ordering::Equal => found_eq += 1,
                Ordering::Less => found_le += 1,
                Ordering::Greater => found_gr += 1,
            }
        }
    }
    (found_eq, found_le, found_gr)
}

#[inline(never)]
fn process_memx_memcmp(texts: &[&str], pattern: &str) -> (usize, usize, usize) {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found_eq: usize = 0;
    let mut found_le: usize = 0;
    let mut found_gr: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let r = memx::memcmp(&line_bytes[i..(i + pat_len)], pat_bytes);
            match r {
                Ordering::Equal => found_eq += 1,
                Ordering::Less => found_le += 1,
                Ordering::Greater => found_gr += 1,
            }
        }
    }
    (found_eq, found_le, found_gr)
}

#[inline(never)]
fn process_memx_memcmp_basic(texts: &[&str], pattern: &str) -> (usize, usize, usize) {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found_eq: usize = 0;
    let mut found_le: usize = 0;
    let mut found_gr: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let r = memx::mem::memcmp_basic(&line_bytes[i..(i + pat_len)], pat_bytes);
            match r {
                Ordering::Equal => found_eq += 1,
                Ordering::Less => found_le += 1,
                Ordering::Greater => found_gr += 1,
            }
        }
    }
    (found_eq, found_le, found_gr)
}

#[inline(never)]
fn process_memx_memcmp_libc(texts: &[&str], pattern: &str) -> (usize, usize, usize) {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found_eq: usize = 0;
    let mut found_le: usize = 0;
    let mut found_gr: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let r = memx::libc::memcmp_libc(&line_bytes[i..(i + pat_len)], pat_bytes);
            match r {
                Ordering::Equal => found_eq += 1,
                Ordering::Less => found_le += 1,
                Ordering::Greater => found_gr += 1,
            }
        }
    }
    (found_eq, found_le, found_gr)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_string_s, match_cnt, less_cnt, greater_count) = create_data::create_data_cmp();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let (n_eq, n_le, n_gr) = process_std_memcmp(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n_eq, match_cnt);
    assert_eq!(n_le, less_cnt);
    assert_eq!(n_gr, greater_count);
    let (n_eq, n_le, n_gr) = process_libc_memcmp(black_box(&vv), black_box(&pat_string));
    assert_eq!(n_eq, match_cnt);
    assert_eq!(n_le, less_cnt);
    assert_eq!(n_gr, greater_count);
    let (n_eq, n_le, n_gr) = process_memx_memcmp(black_box(&vv), black_box(&pat_string));
    assert_eq!(n_eq, match_cnt);
    assert_eq!(n_le, less_cnt);
    assert_eq!(n_gr, greater_count);
    let (n_eq, n_le, n_gr) = process_memx_memcmp_basic(black_box(&vv), black_box(&pat_string));
    assert_eq!(n_eq, match_cnt);
    assert_eq!(n_le, less_cnt);
    assert_eq!(n_gr, greater_count);
    let (n_eq, n_le, n_gr) = process_memx_memcmp_libc(black_box(&vv), black_box(&pat_string));
    assert_eq!(n_eq, match_cnt);
    assert_eq!(n_le, less_cnt);
    assert_eq!(n_gr, greater_count);
    memory_barrier(&vv);
    //
    c.bench_function("std_memcmp", |b| {
        b.iter(|| {
            let _r = process_std_memcmp(black_box(&vv), black_box(pat_string_s));
            memory_barrier(&vv);
        })
    });
    c.bench_function("libc_memcmp", |b| {
        b.iter(|| {
            let _r = process_libc_memcmp(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memcmp", |b| {
        b.iter(|| {
            let _r = process_memx_memcmp(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    /*
    c.bench_function("memx_memcmp_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memcmp_basic(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memcmp_libc", |b| {
        b.iter(|| {
            let _r = process_memx_memcmp_libc(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    */
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(3000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
