#![feature(lang_items, asm, unique, const_fn, naked_functions)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

pub mod mem;
pub mod fs;
#[macro_use]
pub mod dev;
pub mod trap;
pub mod util;

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

#[cfg(debug_assertions)]
fn print_boot_info(boot_info: &multiboot2::BootInformation){
    match boot_info.boot_loader_name_tag() {
        Some(name_tag) => {
            println!("Bootloader: {}", name_tag.name());
        }
        None => {
            println!("No bootloader information provided.");
        }
    };

    println!("Memory area data:");
    for area in boot_info.memory_map_tag()
        .expect("Memory map tag required").memory_areas(){
            println!("    Area start: {:x} length: {:x}", area.base_addr, area.length)
    }
    
    println!("Kernel ELF Sections:");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Kernel ELF sections required.");
    for section in elf_sections_tag.sections(){
            println!("    Section addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
        }
    println!("Kernel and Multiboot ");
    
}

#[cfg(not(debug_assertions))]
fn print_boot_info(boot_info: &multiboot2::BootInformation){
        match boot_info.boot_loader_name_tag() {
        Some(name_tag) => {
            println!("Bootloader: {}", name_tag.name());
        }
        None => {
            println!("No bootloader information provided.");
        }
    };
}

#[allow(dead_code)]
fn log(msg: &str){
    println!("[<TimePlaceholder>] {}",msg);
}

#[no_mangle]
pub extern fn rust_start(mb_info_addr: usize){
    
    let boot_info = unsafe{ multiboot2::load(mb_info_addr) };

    print_build_info();
    print_boot_info(&boot_info);

    // Set up new expandable page table and remap the kernel
    mem::init_mem();

    // Initialize all drivers
    dev::init_io();

    // Initialize file system
    fs::init_fs();

    // Initialize trap handlers
    trap::init_trap();

    unsafe{asm!("hlt")};
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}