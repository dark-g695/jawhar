#![no_std]
#![no_main]

#[path = "mod.rs"]
mod arch;

#[path = "../drives/vga/io.rs"]
mod io;

#[path = "../drives/vga/tty.rs"]
mod tty;

#[path = "../mm/mem.rs"]
mod mem;

/// النوع الوحيد للمعامل الأول: ما يمرره boot.S.
/// x86/x86_64  → magic (في rdi/eax)، والثاني mbi (في rsi/ebx)
/// aarch64     → dtb_ptr (في x0)
/// نستخدم usize ونترك كل معمارية تفسّره.
#[no_mangle]
pub extern "C" fn gdt_init(a0: usize, a1: usize) -> ! {
    let _ = (a0, a1);   // معاملات Multiboot/DTB — تُستخدم لاحقًا
    let _ = mem::memcpy;

    arch::init();
    io::println("jawhar");
    tty::tty();

    arch::halt_forever();
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    arch::halt_forever();
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}
