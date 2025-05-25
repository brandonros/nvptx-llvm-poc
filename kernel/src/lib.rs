#![no_std]
#![no_main]
#![allow(warnings)]
#![feature(abi_ptx)]
#![feature(core_intrinsics)]

mod scopeguard;
mod lock_api;
mod spinning_top;
mod linked_list_allocator;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

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
