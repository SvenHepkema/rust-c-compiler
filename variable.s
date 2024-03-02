 section     .text
global      _start
_start:
mov dword ptr [rsp - 4], rax
mov rax, dword ptr [rsp - 4]
mov rbx, rax
mov rax, 1
int 0x80