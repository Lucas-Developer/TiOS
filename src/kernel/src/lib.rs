#![feature(lang_items, asm)]
#![no_std]

extern crate rlibc;

pub mod mem;
pub mod fs;
pub mod dev;

pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

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

pub fn log_action(s: &str){

}

#[no_mangle]
pub extern fn rust_start(){
    unsafe{clear_console();}
    printk("TiOS (kernel ");
    printk(built_info::PKG_VERSION);
    printk(")  ");
    printk(built_info::TARGET);
    printk("  ");
    printk(built_info::RUSTC_VERSION);
    printk("  ");
    printk(built_info::BUILT_TIME_UTC);
    printk("\n");
    panic!()
    
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}