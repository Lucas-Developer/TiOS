LD=ld
AS=nasm
ASFLAGS=-f elf64
ASMSOURCE=$(wildcard */*.asm)
ASMOBJ=$(patsubst %.asm, %.o, $(ASMSOURCE))


.PHONY: all clean boot

all: $(ASMOBJ) long_boot.o
	$(LD) -n -o boot.o -T arch.ld -r $(ASMOBJ)

long_boot.o: boot/long_boot.c
	cd boot; gcc -ffreestanding -mcmodel=large -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -c long_boot.c -o long_boot.o

%.o: %.asm
	${AS} ${ASFLAGS} $< -o $@

clean:
	rm -rf **/*.o *.o
