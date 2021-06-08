use core::cmp::Ordering;
use libc::{c_int, c_void, size_t};

#[no_mangle]
pub extern "C" fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let c = c as u8;
    match ::memx::memchr(buf, c) {
        Some(pos) => unsafe { buf.as_ptr().add(pos) as *mut c_void },
        None => 0 as *mut c_void,
    }
}

#[no_mangle]
pub extern "C" fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let c = c as u8;
    match ::memx::memrchr(buf, c) {
        Some(pos) => unsafe { buf.as_ptr().add(pos) as *mut c_void },
        None => 0 as *mut c_void,
    }
}

#[no_mangle]
pub extern "C" fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
    let a = unsafe { std::slice::from_raw_parts(cx as *const u8, n) };
    let b = unsafe { std::slice::from_raw_parts(ct as *const u8, n) };
    match ::memx::memcmp(a, b) {
        Ordering::Equal => 0 as c_int,
        Ordering::Less => -1 as c_int,
        Ordering::Greater => 1 as c_int,
    }
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    let a = unsafe { std::slice::from_raw_parts_mut(dest as *mut u8, n) };
    let b = unsafe { std::slice::from_raw_parts(src as *const u8, n) };
    let _r = ::memx::memcpy(a, b);
    dest
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
        None => 0 as *mut c_void,
    }
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    let buf = unsafe { std::slice::from_raw_parts_mut(dest as *mut u8, n) };
    let c = c as u8;
    let _r = ::memx::memset(buf, c, n);
    dest
}
