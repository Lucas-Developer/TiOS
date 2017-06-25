#![feature(lang_items, asm)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn rust_start(){
    panic!()
    
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}