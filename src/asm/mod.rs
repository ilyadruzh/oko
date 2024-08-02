#![feature(asm)]

unsafe fn outb(value: u8, port: u16) {
    asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(value) ::
         "volatile");
}