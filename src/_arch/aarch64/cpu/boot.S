// Kernel Entry Point

.macro ADR_REL register, symbol
	adrp \register, \symbol
	add \register, \register, #:lo12:\symbol
.endm

.section .text._start
//.global _start

_start:
	mov x3, #0x4
	ADR_REL x1, __bss_start
	ADR_REL x2, __bss_end_exclusive
	mov x2, 0
	bl _start_rust // call rust entry point

.L_wait_forever:
	wfe
	b .L_wait_forever

.size	_start, . - _start
.type	_start, function
.global	_start
