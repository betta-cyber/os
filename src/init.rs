use crate::sbi;

global_asm!(include_str!("boot/entry.asm")); // 引入 _start

#[no_mangle]
pub fn rust_main() -> ! {
    crate::interrupt::init();
    crate::clock::init();
    crate::memory::init();
    loop {}
}
