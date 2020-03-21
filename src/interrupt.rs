use crate::clock::{clock_set_next_event, TICK};
use crate::context::TrapFrame;
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::{sscratch, sstatus, stvec};

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0); // 给中断 asm 初始化
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("unexpected trap"),
    }
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn super_timer() {
    // 响应当前时钟中断的同时，手动设置下一个时钟中断
    clock_set_next_event();
    unsafe{
        TICK = TICK + 1;
        if TICK % 100 == 0 {
            println!("100 ticks!");
        }
    }
}
