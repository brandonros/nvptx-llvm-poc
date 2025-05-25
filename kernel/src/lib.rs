#![no_std]
#![no_main]
#![feature(abi_ptx)]
#![feature(core_intrinsics)]

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
