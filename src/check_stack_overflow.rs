#[macro_export]
macro_rules! check_stack_overflow {
    () => {
        check_stack_overflow!(4096);
    };
    ($min_size:expr) => {
        check_stack_overflow!($min_size, false);
    };
    ($min_size:expr, $is_debug: expr) => {
        //#[cfg(feature = "check-stack-overflow")]
        {
            let mut sp: u64;
            let mut bss: u64;
            #[allow(unused_unsafe)]
            unsafe{
                llvm_asm!("mv $0, sp" : "=r"(sp) ::: "volatile");
                llvm_asm!("lui	$0,%hi(_end)\n \
                           addi $0, $0, %lo(_end)" : "=r"(bss) ::: "volatile");
            };
            if $is_debug {
                $crate::debug!("check stack overflow, sp: {}, bss: {} min_size: {}", sp, bss, $min_size);
            }
            if sp < bss + $min_size {
                $crate::syscalls::exit($crate::error::RT_ERROR_STACK_OVERFLOW);
            }
        }
    };
}

