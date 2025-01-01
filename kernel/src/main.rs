#![no_std]
#![no_main]

extern crate alloc;

mod logger;

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


// 出现panic的时候应该怎么处理，标准库std已经写好了。但是在no_std情况下就需要自己写一个handler处理
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_arch = "x86_64")]
bootloader_api::entry_point!(kernel_main);


fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init_heap();
    display_text();
    loop {}
}

fn init_heap() {
    unsafe {
        const HEAP_SIZE: usize = 0x4000;
        let mut heap_space: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        ALLOCATOR.lock().init(heap_space.as_mut_ptr(), HEAP_SIZE);
    }
}

fn display_text() {
    println!("Hello, {}\n", "mini-os");
}
