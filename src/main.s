.pushsection .bootstrap, "ax"
mb2_magic:
    .space 64
mb2_ptr:
    .space 64
.globl _start
_start:
    // Save for OS
    mov [rip +mb2_magic], eax
    mov [rip + mb2_ptr], rbx

    // Load pamOS page map
    mov cr3, rcx

    // Configure stack
    mov rsp, 0x87fff

    // Load GDT included in our pages
    lgdt gdtr_ptr

.reloadCS:
    mov ax, 0
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov edi, [rip + mb2_magic]
    mov rsi, [rip + mb2_ptr]

    movabs rax, offset kmain
    jmp rax
_loop:
    jmp _loop
.popsection

.pushsection .rodata, "a"
gdt:
    .zero 4  // 0th entry null
    // Set Long-mode flag (53), present bit (47), descriptor type bit (44),
    // exec bit (43)
    .quad (1 << 53) | (1 << 47) | (1 << 44) | (1 << 43)
gdtr_ptr:
    // Size - 1
    .word . - gdt - 1
    .quad gdt
.popsection
