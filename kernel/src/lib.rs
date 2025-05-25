#![no_std]
#![no_main]
#![allow(warnings)]
#![feature(abi_ptx)]
#![feature(core_intrinsics)]

use core::{alloc::{GlobalAlloc, Layout}, ffi::c_void};

unsafe extern "C" {
    // implicitly defined by cuda.
    pub fn malloc(size: usize) -> *mut c_void;

    pub fn free(ptr: *mut c_void);
}

pub struct CUDAAllocator;

unsafe impl GlobalAlloc for CUDAAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut _);
    }
}

#[global_allocator]
pub static GLOBAL_ALLOCATOR: CUDAAllocator = CUDAAllocator;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::unreachable() }
}

#[unsafe(no_mangle)]
pub extern "ptx-kernel" fn add(result: *mut i32, a: i32, b: i32) {
    unsafe {
        result.write_volatile(a.unchecked_add(b));
    }
}

#[unsafe(no_mangle)]
pub extern "ptx-kernel" fn ed25519_poc() {
    let mut input = [0u8; 32];
    input[0] = 0x22;
    // TODO: use a real private key
    let scalar = curve25519_dalek::Scalar::from_bytes_mod_order(input);
    let point = curve25519_dalek::constants::ED25519_BASEPOINT_TABLE * &scalar;
    let recip = point.Z.invert();
    let x = &point.X * &recip;
    let y = &point.Y * &recip;
    let mut s: [u8; 32];
    s = y.as_bytes();
    let x_bytes = x.as_bytes();
    let x_is_negative = x_bytes[0] & 1;
    s[31] ^= x_is_negative << 7;
    let compressed_point = curve25519_dalek::edwards::CompressedEdwardsY(s);
    let _public_key_bytes = compressed_point.to_bytes();
}
