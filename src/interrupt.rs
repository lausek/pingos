extern crate x86_64;

use core::fmt::Write;
use interrupt::x86_64::structures::idt::{ExceptionStackFrame, Idt, PageFaultErrorCode};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.divide_by_zero.set_handler_fn(handler);
        idt.debug.set_handler_fn(handler);
        idt.non_maskable_interrupt.set_handler_fn(handler);
        idt.breakpoint.set_handler_fn(handler);
        idt.overflow.set_handler_fn(handler);
        idt.bound_range_exceeded.set_handler_fn(handler);
        idt.invalid_opcode.set_handler_fn(handler);
        idt.device_not_available.set_handler_fn(handler);
        idt.x87_floating_point.set_handler_fn(handler);
        idt.machine_check.set_handler_fn(handler);
        idt.simd_floating_point.set_handler_fn(handler);
        idt.virtualization.set_handler_fn(handler);

        idt.double_fault.set_handler_fn(handler_with_err_code);
        idt.invalid_tss.set_handler_fn(handler_with_err_code);
        idt.segment_not_present.set_handler_fn(handler_with_err_code);
        idt.stack_segment_fault.set_handler_fn(handler_with_err_code);
        idt.general_protection_fault.set_handler_fn(handler_with_err_code);
        idt.alignment_check.set_handler_fn(handler_with_err_code);
        idt.security_exception.set_handler_fn(handler_with_err_code);

        idt.page_fault.set_handler_fn(handler_page_fault);

        idt
    };
}

extern "x86-interrupt" fn handler(stack_frame: &mut ExceptionStackFrame) {
    vga_println!("exception");
}

extern "x86-interrupt" fn handler_page_fault(stack_frame: &mut ExceptionStackFrame, code: PageFaultErrorCode) {
    vga_println!("page fault");
}

extern "x86-interrupt" fn handler_with_err_code(stack_frame: &mut ExceptionStackFrame, code: u64) {
    vga_println!("exception with error code: {}", code);
}

pub fn init() {
    IDT.load();
}
