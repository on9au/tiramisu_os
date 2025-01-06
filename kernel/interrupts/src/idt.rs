use x86_64::structures::idt::InterruptDescriptorTable;
use logging::info;
use lazy_static::lazy_static;

// Define the IST index for double fault
const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Allocate a stack for the double fault handler
#[allow(unused)]
static mut DOUBLE_FAULT_STACK: [u8; 4096] = [0; 4096];

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault.set_handler_fn(gpf_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe { idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(DOUBLE_FAULT_IST_INDEX) };
        // Add more handlers as needed
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    info!("IDT initialized.");
}

extern "x86-interrupt" fn divide_error_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR");
}

extern "x86-interrupt" fn debug_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    panic!("EXCEPTION: DEBUG");
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    panic!("EXCEPTION: BREAKPOINT");
}

extern "x86-interrupt" fn gpf_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT, error code: {}", error_code);
}

extern "x86-interrupt" fn page_fault_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: x86_64::structures::idt::PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT, error code: {:?}", error_code);
}

extern "x86-interrupt" fn double_fault_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT");
}