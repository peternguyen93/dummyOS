// Kernel Entry Point

// https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp \register, \symbol
	add \register, \register, #:lo12:\symbol
.endm

.section .text._start

_start:
	# read boot core
	mrs x1, MPIDR_EL1
	ldr x9, CONST_CORE_ID_MASK
	and x1, x1, x9
	ldr x2, BOOT_CORE_ID
	cmp x1, x2
	b.ne .L_wait_forever

	// If execution reaches here, it is the boot core.
	// Initialize DRAM.
	ADR_REL x0, __bss_start
	ADR_REL x1, __bss_end_exclusive
	// memset bss
.L_bss_init_loop:
	cmp x0, x1
	b.eq .L_prepare_rust
	stp xzr, xzr, [x0], #16
	b .L_bss_init_loop

.L_prepare_rust:
	// Set the stack pointer.
	ADR_REL x0, __boot_core_stack_end_exclusive
	mov sp, x0
	bl _start_rust // call rust entry point

.L_wait_forever:
	wfe
	b .L_wait_forever

.size	_start, . - _start
.type	_start, function
.global	_start
