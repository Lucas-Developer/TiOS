LD=ld
AS=nasm
ASFLAGS=-f elf64
ARCH=${DEFAULT_ARCH}
DEFAULT_ARCH=amd64

.PHONY: all clean arch

all: arch kernel user
	mkdir -p bin
	$(LD) -n -o bin/kernel.bin -T src/kernel.ld src/arch/boot.o 
	cp bin/kernel.bin isofiles/boot
	grub-mkrescue -o os.iso isofiles -d /usr/lib/grub/i386-pc

run: all
	qemu-system-x86_64 -cdrom os.iso -m 64
arch:
	cd src/arch; make ${ARCH}
	
kernel:

user:

clean:
	cd src/arch; make clean
	rm -f bin/*
	rm -f isofiles/boot/kernel.bin
	rm os.iso