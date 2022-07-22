
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;



lazy_static! {
    static ref INTERRUPTDESCRIPTORTABLE: InterruptDescriptorTable = {
       let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
        interrupt_descriptor_table.breakpoint.set_handler_fn(breakpoint_handler);
        interrupt_descriptor_table
    };

}
pub fn init_idt() {
    INTERRUPTDESCRIPTORTABLE.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: \n\tBREAKPOINT : \t{:#?}", stack_frame);
}