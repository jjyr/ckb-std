const MIN_STACK_HEAP_OFFSET: u64 = 4096;

pub fn check_stack_overflow() {
    let mut sp: u64;
    let mut bss: u64;
    unsafe {
        llvm_asm!("mv $0, sp" : "=r"(sp) ::: "volatile");
        llvm_asm!("lui	$0,%hi(_end)\n \
                           addi $0, $0, %lo(_end)" : "=r"(bss) ::: "volatile");
    };
    #[cfg(feature = "debug-check-stack-overflow")]
    crate::debug!(
        "check stack overflow, sp: {}, bss: {} min_size: {}",
        sp,
        bss,
        MIN_STACK_HEAP_OFFSET
    );
    if sp < bss + MIN_STACK_HEAP_OFFSET {
        crate::syscalls::exit(crate::error::RT_ERROR_STACK_OVERFLOW);
    }
}
