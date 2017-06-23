LD=ld
AS=nasm
ASFLAGS=-f elf64
ASMSOURCE=$(wildcard */*.asm)
ASMOBJ=$(patsubst %.asm, %.o, $(ASMSOURCE))


.PHONY: all clean boot

all: $(ASMOBJ)
	$(LD) -n -o boot.o -T arch.ld $(ASMOBJ)

%.o: %.asm
	${AS} ${ASFLAGS} $< -o $@

clean:
	rm -rf **/*.o *.o
