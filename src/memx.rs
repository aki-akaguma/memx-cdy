use libc::{c_int, c_void, size_t};

extern {
    fn _memcpy_impl(dest: *mut u8, src: *const u8, n: size_t);
    fn _memset_impl(cx: *mut u8, c: u8, n: size_t);
    //
    fn _memchr_impl(cx: *const u8, c: u8, n: size_t) -> *const u8;
    fn _memrchr_impl(cx: *const u8, c: u8, n: size_t) -> *const u8;
    fn _memcmp_impl(cx: *const u8, ct: *const u8, n: size_t) -> c_int;
    fn _memeq_impl(cx: *const u8, ct: *const u8, n: size_t) -> c_int;
    fn _memmem_impl(haystack: *const u8, haystacklen: size_t, needle: *const u8, needlelen: size_t) -> *const u8;
    fn _memrmem_impl(haystack: *const u8, haystacklen: size_t, needle: *const u8, needlelen: size_t) -> *const u8;
}

pub fn memchr(buf: &[u8], c: u8) -> Option<usize> {
    let cx = buf.as_ptr();
    let len = buf.len();
    let r = unsafe { _memchr_impl(cx, c, len as size_t) };
    if r.is_null() {
        None
    } else {
        Some(r as usize - cx as usize)
    }
}
