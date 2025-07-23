#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

global_asm!(include_str!("asm/boot.s"), options(raw));

#[allow(non_camel_case_types)]
pub enum SBIError {
    SBI_ERR_FAILED,
    SBI_ERR_NOT_SUPPORTED,
    SBI_ERR_INVALID_PARAM,
    SBI_ERR_DENIED,
    SBI_ERR_INVALID_ADDRESS,
    SBI_ERR_ALREADY_AVAILABLE,
    SBI_ERR_ALREADY_STARTED,
    SBI_ERR_ALREADY_STOPPED,
    SBI_ERR_NO_SHMEM,
    UNKNOWN_ERROR,
}

impl SBIError {
    pub fn new(error: isize) -> Self {
        match error {
            -1 => SBIError::SBI_ERR_FAILED,
            -2 => SBIError::SBI_ERR_NOT_SUPPORTED,
            -3 => SBIError::SBI_ERR_INVALID_PARAM,
            -4 => SBIError::SBI_ERR_DENIED,
            -5 => SBIError::SBI_ERR_INVALID_ADDRESS,
            -6 => SBIError::SBI_ERR_ALREADY_AVAILABLE,
            -7 => SBIError::SBI_ERR_ALREADY_STARTED,
            -8 => SBIError::SBI_ERR_ALREADY_STARTED,
            -9 => SBIError::SBI_ERR_NO_SHMEM,
            _ => SBIError::UNKNOWN_ERROR,
        }
    }
}

#[unsafe(no_mangle)]
pub fn print_uart(text: &str) -> Result<usize, SBIError> {
    unsafe {
        let error: isize;
        let value: usize;

        asm!(
            "ecall",
            inlateout("a0") text.len() => error,
            inlateout("a1") text.as_ptr() => value,
            in("a2") 0,
            in("a6") 0,
            in("a7") 0x4442434e,
        );

        match error {
            0 => Ok(value),
            _ => Err(SBIError::new(error)),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    if print_uart("Hello Rust using SBI!\n").is_err() {};
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
