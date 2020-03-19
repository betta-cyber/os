use crate::sbi;

global_asm!(include_str!("boot/entry.asm")); // 引入 _start

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub fn rust_main() -> ! {
    for &c in HELLO {
        sbi::console_putchar(c as usize);
    }
    loop {}
}
