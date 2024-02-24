gcc-%: c-source/%.c
	gcc $^ -o gcc-binaries/$*

rcc-%: c-source/%.c
	cargo run -- $^ generated-asm/$*.asm
	make $*.asm

%.asm: generated-asm/%.asm
	nasm -f elf64 $^ -O0 -o obj/$*.o
	ld -s -o rust-binaries/$* obj/$*.o 

clean:
	rm generated-asm/* -f
	rm obj/* -f
	rm rust-binaries/* -f
	rm gcc-binaries/* -f

