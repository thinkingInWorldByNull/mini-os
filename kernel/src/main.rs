#![no_std]
#![no_main]  // disable rust level entry point

mod logger;

use crate::logger::Logger;
use core::fmt::Write;
use core::panic::PanicInfo;

// 出现panic的时候应该怎么处理，标准库std已经写好了。但是在no_std情况下就需要自己写一个handler处理
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
bootloader_api::entry_point!(kernel_main);


fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    show(_boot_info);
    loop {}
}

fn show(boot_info: &'static mut bootloader_api::BootInfo) {
    init_logger(boot_info).expect("init log error")
        .write_str("Hello, world!\n")
        .expect("write error, please check");
}

fn init_logger(boot_info: &'static mut bootloader_api::BootInfo) -> Option<Logger>{
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let buf_info = framebuffer.info().clone();

        Some(Logger::new(framebuffer.buffer_mut(), buf_info))
    } else {
        None
    }
}