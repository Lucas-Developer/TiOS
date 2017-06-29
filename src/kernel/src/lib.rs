#![feature(alloc, lang_items, asm, unique, const_fn, naked_functions, core_intrinsics)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate lazy_static;
extern crate x86_64;
#[macro_use]
extern crate bitflags;

/* Temporary heap allocator crate */
extern crate bump_allocator;
#[macro_use]
extern crate alloc;

#[macro_use]
pub mod dev;
pub mod fs;
pub mod mem;
pub mod procs;
pub mod trap;
pub mod util;


pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn print_build_info(){
    println!("TiOS kernel version {} ( {} , {} ) {} {}\n",
             built_info::PKG_VERSION,
             built_info::TARGET,
             built_info::HOST,
             built_info::RUSTC_VERSION,
             built_info::BUILT_TIME_UTC);
    #[cfg(debug_assertions)]
    {
        println!("Debug build. DO NOT USE FOR RELEASE.\n");
    }
}

use spin::Mutex;
use dev::clock::DateTime;
pub static BOOT_TIME: Mutex<DateTime> = Mutex::new(DateTime{
    year: 00,
    month: 1,
    day: 1,
    hour: 0,
    min: 0,
    sec: 0,
});

fn print_boot_info(boot_info: &multiboot2::BootInformation){
    println!("System boot info:");
    match boot_info.boot_loader_name_tag() {
        Some(name_tag) => {
            println!("Bootloader: {}", name_tag.name());
        }
        None => {
            println!("No bootloader information provided.");
        }
    };

    let rtctime = dev::clock::RTC.lock().read_rtc();
    println!("CMOS clock: {:?}", rtctime);
    BOOT_TIME.lock().update(rtctime);
    

    #[cfg(debug_assertions)]
    {
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

        let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
            .min().unwrap();
        let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
            .max().unwrap();
        let multiboot_start = boot_info.start_address();
        let multiboot_end = boot_info.end_address();
        println!("Kernel start : {:x}, end : {:x}", kernel_start, kernel_end);
        println!("Multiboot start : {:x}, end : {:x}", multiboot_start, multiboot_end);
    }
    println!("");
}

#[allow(dead_code)]
fn log(msg: &str){
    let boot_seconds = dev::clock::RTC.lock().read_rtc() - *(BOOT_TIME.lock());
    println!("[{:08}] {}",boot_seconds,msg);
}

#[allow(dead_code)]
fn log_status(msg:&str, res: Result<(),isize>){
    let boot_seconds = dev::clock::RTC.lock().read_rtc() - *(BOOT_TIME.lock());
    use dev::console::{ColorCode,ConsoleColor};
    let good_color = ColorCode::new(ConsoleColor::Green, ConsoleColor::Black);
    let bad_color = ColorCode::new(ConsoleColor::White, ConsoleColor::Red);
    print_color!(good_color, "[{:08}] ", boot_seconds);
    print!("{}  ", msg);
    match res {
        Ok(_) => {
            print_color!(good_color, "[OK]");
        }
        Err(code) => {
            print_color!(bad_color, "[Err: 0x{:x}]", code);
        }
    };
    println!("");
}

fn sys_halt(code: usize) -> ! {
    println!("Code {}",code);
    log("System halted.");
    unsafe{asm!("cli")};
    unsafe{asm!("hlt")};
    panic!()
}

#[no_mangle]
pub extern fn rust_start(mb_info_addr: usize){
    
    let boot_info = unsafe{ multiboot2::load(mb_info_addr) };

    print_build_info();
    print_boot_info(&boot_info);

    util::enable_nxe_bit();
    util::enable_write_protect_bit();

    // Set up new expandable page table and remap the kernel
    mem::init_mem(&boot_info);

    // Initialize trap handlers
    trap::init_trap();

    // Initialize all drivers
    dev::init_io();

    // Initialize file system
    fs::init_fs();

    sys_halt(0);
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