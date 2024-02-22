add-two-numbers: c-source/add-two-numbers.c
	gcc $^ -o c-binaries/$@

function-call: c-source/function-call.c
	gcc $^ -o c-binaries/$@

return-ten: c-source/return-ten.c
	gcc $^ -o c-binaries/$@

all-c-binaries: add-two-numbers function-call return-ten
