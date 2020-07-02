fn main() {
    cc::Build::new()
        .file("src/asm/asm.S")
        .compile("ckb-asm");
}
