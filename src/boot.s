
.section .text._start

_start:
	mrs	x0, MPIDR_EL1 /* Read multiprocessor affinity register */
	and	x0, x0, {CONST_CORE_ID_MASK} /* Mask core id info */

	/* If we're not on the boot core, wait indefinitely */
	ldr	x1, BOOT_CORE_ID
	cmp	x0, x1
	b.ne	.do_nothing

	/* Grab start and end of uninitialized data section */
	adrp    x0, __bss_start
	add     x0, x0, #:lo12:__bss_start
	adrp    x1, __bss_end
	add     x1, x1, #:lo12:__bss_end

.zero_uninitialized_data:
	cmp	x0, x1
	b.eq	.set_stack_pointer
	stp	xzr, xzr, [x0], #16
	b	.zero_uninitialized_data

.set_stack_pointer:
	adrp    x0, __boot_core_stack_end
	add     x0, x0, #:lo12:__boot_core_stack_end
	mov	sp, x0

	/* Calls our entry point rust function */
	b	_start_rust

.do_nothing:
	wfe
	b	.do_nothing

/* set _start metadata for the linker */
.size	_start, . - _start
.type	_start, function
.global	_start
