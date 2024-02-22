add-two-numbers: c-source/add-two-numbers.c
	gcc $^ -o gcc-binaries/$@

function-call: c-source/function-call.c
	gcc $^ -o gcc-binaries/$@

return-ten: c-source/return-ten.c
	gcc $^ -o gcc-binaries/$@

all-c-binaries: add-two-numbers function-call return-ten
