use x86_64::structures::idt::InterruptDescriptorTable;
use logging::{info, warn};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault.set_handler_fn(gpf_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        // Add more handlers as needed
        idt
    };
}

pub(crate) fn init_idt() {
    IDT.load();
    info!("IDT initialized.");
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    warn!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    warn!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn gpf_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: x86_64::structures::idt::PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT {:?}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: x86_64::structures::idt::InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}