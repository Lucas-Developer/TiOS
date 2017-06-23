LD=ld 
AS=nasm
ASFLAGS=-f elf64
DIRS=$(patsubst %/, %_clean, $(wildcard */))

.PHONY: all clean amd64

all:
	@echo "Arch module build error: Need to specify target architecture."
	@exit 2
amd64:
	cd amd64; make
	cp amd64/boot.o ./
clean: $(DIRS)
	@echo $(DIRS)
	rm -f *.o
%_clean:
	cd $(patsubst %_clean, %, $@); make clean	
