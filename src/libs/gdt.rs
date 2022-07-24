use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::gdt::SegmentSelector;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut task_state_segment = TaskStateSegment::new();
        task_state_segment.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        task_state_segment
    };

    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable,Selectors) = {
        let mut global_descriptor_table = GlobalDescriptorTable::new();
        let code_selector = global_descriptor_table.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = global_descriptor_table.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        (global_descriptor_table, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GLOBAL_DESCRIPTOR_TABLE.0.load();
    unsafe {
        CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code_selector);
        load_tss(GLOBAL_DESCRIPTOR_TABLE.1.tss_selector);
    }
}