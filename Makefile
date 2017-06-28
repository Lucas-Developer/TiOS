LD=ld
AS=nasm
ASFLAGS=-f elf64
ARCH=${DEFAULT_ARCH}
DEFAULT_ARCH=amd64
TARGET=x86_64-TiOS
debug: RELEASE_ARGS=
debug: LIB_PATH=debug
release: RELEASE_ARGS=--release
release: LIB_PATH=release

.PHONY: all clean arch kernel release debug

all: debug

debug: os.iso

release: os.iso

os.iso: arch kernel user
	mkdir -p bin
	$(LD) -n --gc-sections -o bin/kernel.bin -T src/kernel.ld src/arch/boot.o src/kernel/target/$(TARGET)/$(LIB_PATH)/libkernel.a
	cp bin/kernel.bin isofiles/boot
	grub-mkrescue -o os.iso isofiles -d /usr/lib/grub/i386-pc

run: debug
	qemu-system-x86_64 -cdrom os.iso -m 64 -s
run-release: release
	qemu-system-x86_64 -cdrom os.iso -m 64
gdb:
	gdb "bin/kernel.bin" -ex "target remote :1234"
arch:
	cd src/arch; make ${ARCH}
kernel:
	cd src/kernel; xargo build --target $(TARGET) $(RELEASE_ARGS)
user:

clean:
	cd src/arch; make clean
	cd src/kernel; xargo clean
	rm -f bin/*
	rm -f isofiles/boot/kernel.bin
	rm -f os.iso