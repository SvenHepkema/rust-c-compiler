add-two-numbers: c-source/add-two-numbers.c
	gcc $^ -o gcc-binaries/$@

function-call: c-source/function-call.c
	gcc $^ -o gcc-binaries/$@

return-ten: c-source/return-ten.c
	gcc $^ -o gcc-binaries/$@

all-c-binaries: add-two-numbers function-call return-ten

%.asm: generated-asm/%.asm
	nasm -f elf64 $^ -O0 -o obj/$*.o
	ld -s -o rust-binaries/$* obj/$*.o 

clean:
	rm generated-asm/* -f
	rm obj/* -f
	rm rust-binaries/* -f
