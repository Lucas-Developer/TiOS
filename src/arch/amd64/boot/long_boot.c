

void rust_start(long addr);

void bootstrap_long(long addr) {
    rust_start(addr);
}