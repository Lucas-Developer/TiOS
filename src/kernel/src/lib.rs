#![feature(lang_items, asm, unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;

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

fn print_build_info(){
    printk("TiOS (kernel ");
    printk(built_info::PKG_VERSION);
    printk(")  ");
    printk(built_info::TARGET);
    printk("  ");
    printk(built_info::RUSTC_VERSION);
    printk("  ");
    printk(built_info::HOST);
    printk("  ");
    printk(built_info::BUILT_TIME_UTC);
    printk("\n");
}

fn test_console(){
    use dev::console::*;
    let mut c = Console::new(ColorCode::new(ConsoleColor::LightGray, ConsoleColor::Black), 0xb8000);
    //c.clear_screen();
    c.write_str("Test completed. Good line switching and scrolling for the text console under Rust.\n\n\n");
}

#[no_mangle]
pub extern fn rust_start(){
    unsafe{clear_console();}
    print_build_info();
    test_console();
    panic!()
    
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}