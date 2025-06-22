#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

global_asm!(include_str!("asm/boot.s"), options(raw));

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    print_uart("Hello Rust using SBI!\n");
    loop {}
}

#[unsafe(no_mangle)]
fn print_uart(text: &str) {
    unsafe {
        asm!(
            "ecall",
            in("a7") 0x4442434e,
            in("a6") 0,
            in("a0") text.len(),
            in("a1") text.as_ptr(),
            in("a2") 0,
        )
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
