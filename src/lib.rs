use libc::{c_int, c_void, size_t};

#[link(name = "memx_c_2")]
extern {
    fn _memcpy_basic_impl(dest: *mut u8, src: *const u8, n: usize);
    fn _memset_basic_impl(cx: *mut u8, n: usize, c: u8);
    //
    fn _memchr_basic_impl(cx: *const u8, n: usize, c: u8) -> *const u8;
    fn _memrchr_basic_impl(cx: *const u8, n: usize, c: u8) -> *const u8;
    //
    fn _memcmp_basic_impl(cx: *const u8, ct: *const u8, n: usize) -> i32;
    fn _memeq_basic_impl(cx: *const u8, ct: *const u8, n: usize) -> i32;
    fn _memmem_basic_impl(haystack: *const u8, hay_len: usize, needle: *const u8, nee_len: usize) -> *const u8;
    fn _memrmem_basic_impl(haystack: *const u8, hay_len: usize, needle: *const u8, nee_len: usize) -> *const u8;
}

#[link(name = "memx_c_simd")]
extern {
    //
    fn _memchr_sse2_impl(cx: *const u8, n: usize, c: u8) -> *const u8;
    fn _memrchr_sse2_impl(cx: *const u8, n: usize, c: u8) -> *const u8;
}

////

#[no_mangle]
pub extern "C" fn memchr_basic(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    let r = unsafe { _memchr_basic_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memchr_sse2(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    let r = unsafe { _memchr_sse2_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memrchr_basic(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    let r = unsafe { _memrchr_basic_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memrchr_sse2(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    let r = unsafe { _memrchr_sse2_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

////

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    unsafe { _memcpy_basic_impl(dest as *mut u8, src as *const u8, n) };
    dest
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    unsafe { _memset_basic_impl(dest as *mut u8, n, c as u8) };
    dest
}

////

#[no_mangle]
pub extern "C" fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    //let r = unsafe { _memchr_basic_impl(cx as *const u8, n, c as u8) };
    let r = unsafe { _memchr_sse2_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *const c_void {
    //let r = unsafe { _memrchr_basic_impl(cx as *const u8, n, c as u8) };
    let r = unsafe { _memrchr_sse2_impl(cx as *const u8, n, c as u8) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
    let r = unsafe { _memcmp_basic_impl(cx as *const u8, ct as *const u8, n) };
    r
}

#[no_mangle]
pub extern "C" fn bcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
    let r = unsafe { _memeq_basic_impl(cx as *const u8, ct as *const u8, n) };
    r
}

#[no_mangle]
pub extern "C" fn memmem(
    haystack: *const c_void,
    hay_len: size_t,
    needle: *const c_void,
    nee_len: size_t,
) -> *const c_void {
    let r = unsafe { _memmem_basic_impl(haystack as *const u8, hay_len, needle as *const u8, nee_len) };
    r as *const c_void
}

#[no_mangle]
pub extern "C" fn memrmem(
    haystack: *const c_void,
    hay_len: size_t,
    needle: *const c_void,
    nee_len: size_t,
) -> *const c_void {
    let r = unsafe { _memrmem_basic_impl(haystack as *const u8, hay_len, needle as *const u8, nee_len) };
    r as *const c_void
}

//pub mod memx;

/*
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
*/
