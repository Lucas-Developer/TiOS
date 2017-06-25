LD=ld
AS=nasm
ASFLAGS=-f elf64
ARCH=${DEFAULT_ARCH}
DEFAULT_ARCH=amd64
TARGET=x86_64-TiOS

.PHONY: all clean arch

all: arch kernel user
	mkdir -p bin
	$(LD) -n --gc-sections -o bin/kernel.bin -T src/kernel.ld src/arch/boot.o src/kernel/target/$(TARGET)/debug/libkernel.a
	cp bin/kernel.bin isofiles/boot
	grub-mkrescue -o os.iso isofiles -d /usr/lib/grub/i386-pc

run: all
	qemu-system-x86_64 -cdrom os.iso -m 64
arch:
	cd src/arch; make ${ARCH}
kernel:
	cd src/kernel; xargo build --target $(TARGET)
user:

clean:
	cd src/arch; make clean
	cd src/kernel; xargo clean
	rm -f bin/*
	rm -f isofiles/boot/kernel.bin
	rm -f os.iso