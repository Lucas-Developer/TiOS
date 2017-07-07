

void rust_start(long addr);

void bootstrap_long(long addr) {
    __asm__ ("movabs $0xffff800000000000, %rax\n\t"
             "add %rax, %rsp\n\t"
             "add %rax, %rbp\n\t");
    rust_start(addr);
}