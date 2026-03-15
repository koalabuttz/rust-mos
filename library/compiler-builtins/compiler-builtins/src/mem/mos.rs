// Byte-by-byte memory operations for the MOS 6502.
//
// The 6502 is an 8-bit CPU with no native word-load instructions wider than
// a byte, so the word-at-a-time optimisations in `impls.rs` (alignment
// checks, shift-mask reassembly, partial loads) compile to *more* code than
// a simple byte loop — adding ~1,400 bytes of .text for no speed benefit.
//
// These implementations match the pre-v0.1.130 compiler-builtins behaviour
// that produced 52 B memcpy / 145 B memmove on this target.

use core::ffi::c_int;

#[inline(always)]
pub unsafe fn copy_forward(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        *dest.wrapping_add(i) = *src.wrapping_add(i);
        i += 1;
    }
}

#[inline(always)]
pub unsafe fn copy_backward(dest: *mut u8, src: *const u8, n: usize) {
    // Use pointer decrement (sub) instead of base+offset (wrapping_add).
    // On 6502, wrapping_add(i) recomputes the 16-bit address each iteration.
    // sub(1) lets the compiler use the non-wrapping assumption for tighter code.
    // This matches the mrk-its/compiler-builtins mos-0.1.108 approach.
    let mut s = src.add(n);
    let mut d = dest.add(n);
    let dest_start = dest;
    while dest_start < d {
        d = d.sub(1);
        s = s.sub(1);
        *d = *s;
    }
}

#[inline(always)]
pub unsafe fn set_bytes(s: *mut u8, c: u8, n: usize) {
    let mut i = 0;
    while i < n {
        *s.wrapping_add(i) = c;
        i += 1;
    }
}

#[inline(always)]
pub unsafe fn compare_bytes(s1: *const u8, s2: *const u8, n: usize) -> c_int {
    let mut i = 0;
    while i < n {
        let a = *s1.wrapping_add(i);
        let b = *s2.wrapping_add(i);
        if a != b {
            return c_int::from(a) - c_int::from(b);
        }
        i += 1;
    }
    0
}

#[inline(always)]
pub unsafe fn c_string_length(mut s: *const core::ffi::c_char) -> usize {
    let mut n = 0;
    while *s != 0 {
        n += 1;
        s = s.wrapping_add(1);
    }
    n
}
