mod gdt;

use x86_64::structures::idt::Idt;
use mem::MemoryController;
use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::instructions::interrupts::*;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use x86_64::instructions::segmentation::set_cs;
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::SegmentSelector;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

use spin::Once;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

pub fn init_trap(memory_controller: &mut MemoryController) {

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
            double_fault_stack.top());
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });
    gdt.load();

    unsafe {
        // reload code segment register
        set_cs(code_selector);
        // load TSS
        load_tss(tss_selector);
    }
    
    IDT.load();
    
}