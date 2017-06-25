#![feature(lang_items, asm, unique, const_fn)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

pub mod mem;
pub mod fs;
#[macro_use]
pub mod dev;
pub mod trap;

pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn print_build_info(){
    println!("TiOS kernel version {} ( {} , {} ) {} {}",
             built_info::PKG_VERSION,
             built_info::TARGET,
             built_info::HOST,
             built_info::RUSTC_VERSION,
             built_info::BUILT_TIME_UTC);
}

#[allow(dead_code)]
fn log(msg: &str){
    println!("[<TimePlaceholder>] {}",msg);
}

#[allow(dead_code)]
fn log_error(msg: &str){
    use dev::console::*;
    CONSOLE.lock().change_color_code(ColorCode::new(ConsoleColor::White, ConsoleColor::Red));
    println!("[<TimePlaceholder>] {}",msg);
    CONSOLE.lock().change_color_code(ColorCode::new(ConsoleColor::LightGray, ConsoleColor::Black));
}

#[no_mangle]
pub extern fn rust_start(){
    print_build_info();
    println!("");

    // Set up new expandable page table and remap the kernel
    log("Initializing memory subsystems...");
    mem::init_mem();

    // Initialize file system
    log("Initializing filesystem...");
    fs::init_fs();

    // Initialize trap handlers
    log("Initializing traps...");
    trap::init_trap();

    unsafe{asm!("hlt")};
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}