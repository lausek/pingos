extern crate x86_64;

use interrupt::x86_64::structures::idt::{ExceptionStackFrame, Idt};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    //log!("breakpoint reached"); 
}

pub fn init() {
    IDT.load();
}
