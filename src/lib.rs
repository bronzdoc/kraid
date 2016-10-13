#![feature(lang_items)]
#![no_std]

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    unsafe {
        let vga1 = 0xb8000 as *mut u64;
        let vga2 = 0xb8008 as *mut u64;

        *vga1 = 0x2f692f612f722f4b;
        *vga2 = 0x2f64
    };

    loop { }
}
