/*!
c interface of [`memx`](https://crates.io/crates/memx): the fast memory functions like libc memcmp(), memchr(), memmem(), memcpy(), memset()

# Features

* Faster os libc and musl libc
* Replace dynamic os libc using LD_PRELOAD

# Quick install

1. You can install this into cargo lib path:

```text
cargo build --release
mkdir -p $HOME/.cargo/lib
cp -a target/release/libmemx_cdy.so target/release/libmemx_cdy.a $HOME/.cargo/lib
```

2. You can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Usage
Easy to use, LD_PRELOAD.

1. usecase: cargo install

```text
$ LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so command
```

or

```text
$ export LD_PRELOAD=~/.cargo/lib/libmemx_cdy.so
```

2. usecase: debian package

```text
$ LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1 command
```

or

```text
$ export LD_PRELOAD=/usr/lib/libmemx_cdy.so.0.1
```

# 2-step of using on your rust source code

1. add to dependences of Cargo.toml:

```text
[dependencies]
memx-cdy = "0.1"
```

2. call `memx_init()` in main function

```text
fn main() {
    memx_cdy::memx_init();
    //
    // follow your code
}
```

*/
use libc::{c_int, c_void, size_t};

/// This is the dummy function fot the easy linking.
///
/// This `memx_init()` function do nothing. However, links are simplified.
///
/// # Example:
/// In your binary package, the main function is like this.
/// ```text
/// fn main () {
///     memx_cdy::memx_init(); // fast mem operation.
/// }
/// ```
/// In your library package, good with any function.
/// ```
/// fn xxx_func () {
///     memx_cdy::memx_init(); // fast mem operation.
/// }
/// ```
///
#[no_mangle]
pub extern "C" fn memx_init() {
    // Nothing todo
}

#[no_mangle]
pub extern "C" fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let c = c as u8;
    match ::memx::memchr(buf, c) {
        Some(pos) => unsafe { buf.as_ptr().add(pos) as *mut c_void },
        None => std::ptr::null_mut::<c_void>(),
    }
}

#[no_mangle]
pub extern "C" fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let c = c as u8;
    match ::memx::memrchr(buf, c) {
        Some(pos) => unsafe { buf.as_ptr().add(pos) as *mut c_void },
        None => std::ptr::null_mut::<c_void>(),
    }
}

use core::cmp::Ordering;

#[no_mangle]
pub extern "C" fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
    let a = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let b = unsafe { std::slice::from_raw_parts(ct as *const u8, n) };
    let r = if n < 4 {
        if n == 0 {
            Ordering::Equal
        } else {
            let mut rr = a[0].cmp(&b[0]);
            for x in 1..n {
                rr = if rr == Ordering::Equal {
                    a[x].cmp(&b[x])
                } else {
                    rr
                };
            }
            rr
        }
    } else {
        ::memx::memcmp(a, b)
    };
    match r {
        Ordering::Equal => 0 as c_int,
        Ordering::Less => -1 as c_int,
        Ordering::Greater => 1 as c_int,
    }
}

#[no_mangle]
pub extern "C" fn bcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
    let a = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let b = unsafe { std::slice::from_raw_parts(ct as *const u8, n) };
    if ::memx::memeq(a, b) {
        0 as c_int
    } else {
        -1 as c_int
    }
}

#[no_mangle]
pub extern "C" fn memmem(
    haystack: *const c_void,
    haystacklen: size_t,
    needle: *const c_void,
    needlelen: size_t,
) -> *mut c_void {
    let haystack = unsafe { std::slice::from_raw_parts(haystack as *const u8, haystacklen) };
    let needle = unsafe { std::slice::from_raw_parts(needle as *const u8, needlelen) };
    match ::memx::memmem(haystack, needle) {
        Some(pos) => unsafe { haystack.as_ptr().add(pos) as *mut c_void },
        None => std::ptr::null_mut::<c_void>(),
    }
}

// bug: in the case that target is "xxx-musl",
// bug: ld: error: duplicate symbol: memcpy
#[cfg(all(not(target_env = "musl"), not(feature = "no_memcpy")))]
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    let a = unsafe { std::slice::from_raw_parts_mut(dest as *mut u8, n) };
    let b = unsafe { std::slice::from_raw_parts(src as *const u8, n) };
    let _r = ::memx::memcpy(a, b);
    dest
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts_mut(dest as *mut u8, n) };
    let c = c as u8;
    ::memx::memset(buf, c);
    dest
}

#[cfg(test)]
mod test {
    #[test]
    fn test_memcmp_00() {
        let pat_1 = vec![b'A', b'B', b'C'];
        let pat_2 = vec![b'A', b'B', b'C'];
        let pat_3 = vec![b'A', b'B', b'c'];
        let p1 = pat_1.as_slice().as_ptr() as *const libc::c_void;
        let p2 = pat_2.as_slice().as_ptr() as *const libc::c_void;
        let p3 = pat_3.as_slice().as_ptr() as *const libc::c_void;
        //
        let r = super::memcmp(p1, p2, 3);
        assert_eq!(r, 0);
        let r = super::memcmp(p1, p3, 3);
        assert_eq!(r, -1);
    }
    #[test]
    fn test_memcmp_01() {
        let pat_1 = vec![b'A', b'B', b'C', b'D'];
        let pat_2 = vec![b'A', b'B', b'C', b'D'];
        let pat_3 = vec![b'A', b'B', b'c', b'D'];
        let p1 = pat_1.as_slice().as_ptr() as *const libc::c_void;
        let p2 = pat_2.as_slice().as_ptr() as *const libc::c_void;
        let p3 = pat_3.as_slice().as_ptr() as *const libc::c_void;
        //
        let r = super::memcmp(p1, p2, 4);
        assert_eq!(r, 0);
        let r = super::memcmp(p1, p3, 4);
        assert_eq!(r, -1);
    }
}
