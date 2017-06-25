#![feature(lang_items, asm)]
#![no_std]

extern crate rlibc;

pub mod mem;
pub mod fs;

// Temoporary printk functions
extern "C" {
    fn clear_console();
    fn print_char(word:u16);
}

fn printk(s: &str){
    for c in s.chars(){
        unsafe{
            print_char(c as u16 | 0x0700 as u16);
        }
    }
}

#[no_mangle]
pub extern fn rust_start(){
    printk("TiOS (kernel 0.1.0)\n");
    panic!()
    
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}