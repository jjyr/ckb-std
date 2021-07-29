use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let mut build = cc::Build::new();
    // build
    //     .file("dl-c-impl/lib.c")
    //     .static_flag(true)
    //     .flag("-O3")
    //     .flag("-fno-builtin-printf")
    //     .flag("-fno-builtin-memcmp")
    //     .flag("-nostdinc")
    //     .flag("-nostdlib")
    //     .flag("-fvisibility=hidden")
    //     .flag("-fdata-sections")
    //     .flag("-ffunction-sections")
    //     .include("dl-c-impl/ckb-c-stdlib")
    //     .include("dl-c-impl/ckb-c-stdlib/libc")
    //     .flag("-Wall")
    //     .flag("-Werror")
    //     .flag("-Wno-unused-parameter")
    //     .flag("-Wno-nonnull")
    //     .define("__SHARED_LIBRARY__", None);
    // ckb-std only supports riscv64 target arch
    // but we can still use cargo check under other archs
    if target_arch == "riscv64" {
        cc::Build::new()
            .file("src/asm/syscall.S")
            .compile("ckb-syscall");
        // build.
        //     flag("-Wno-nonnull-compare")
        //     .flag("-nostartfiles")
        //     .compile("dl-c-impl");
    } else {
        build.define("CKB_STDLIB_NO_SYSCALL_IMPL", None);
        // build.compile("dl-c-impl");
    }
}
