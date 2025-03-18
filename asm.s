	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"asm.6616e5629e0ebbc4-cgu.0"
	.def	_ZN3std2rt10lang_start17hde7798f4d69721a4E;
	.scl	2;
	.type	32;
	.endef
	.globl	_ZN3std2rt10lang_start17hde7798f4d69721a4E
	.p2align	4, 0x90
_ZN3std2rt10lang_start17hde7798f4d69721a4E:
.seh_proc _ZN3std2rt10lang_start17hde7798f4d69721a4E
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movb	%r9b, %al
	movq	%r8, %r9
	movq	%rdx, %r8
	movq	%rcx, 48(%rsp)
	leaq	48(%rsp), %rcx
	leaq	.L__unnamed_1(%rip), %rdx
	movb	%al, 32(%rsp)
	callq	_ZN3std2rt19lang_start_internal17h69d115c24f582c39E
	movq	%rax, 40(%rsp)
	movq	40(%rsp), %rax
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h899718a6375bf6c9E
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7cb5d6b883c9d05fE
	movl	%eax, 36(%rsp)
	movl	36(%rsp), %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h899718a6375bf6c9E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h899718a6375bf6c9E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17h899718a6375bf6c9E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	_ZN4core3ops8function6FnOnce9call_once17h486b30ddf29972c8E
	#APP
	#NO_APP
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1fea5d23bba74984E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1fea5d23bba74984E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1fea5d23bba74984E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN4core3ops8function6FnOnce9call_once17h4103446a3b95fa77E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce9call_once17h4103446a3b95fa77E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce9call_once17h4103446a3b95fa77E:
.Lfunc_begin0:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17h4103446a3b95fa77E
	.seh_handler rust_eh_personality, @unwind, @except
	subq	$72, %rsp
	.seh_stackalloc 72
	.seh_endprologue
	movq	%rcx, 40(%rsp)
.Ltmp0:
	leaq	40(%rsp), %rcx
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E
.Ltmp1:
	movl	%eax, 36(%rsp)
	jmp	.LBB4_3
.LBB4_1:
	movq	56(%rsp), %rcx
	callq	_Unwind_Resume
.LBB4_2:
.Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 56(%rsp)
	movl	%eax, 64(%rsp)
	jmp	.LBB4_1
.LBB4_3:
	movl	36(%rsp), %eax
	addq	$72, %rsp
	retq
.Lfunc_end0:
	.seh_handlerdata
	.text
	.seh_endproc
	.section	.xdata,"dr"
	.p2align	2, 0x0
GCC_except_table4:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end0-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0
	.text

	.def	_ZN4core3ops8function6FnOnce9call_once17h486b30ddf29972c8E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce9call_once17h486b30ddf29972c8E:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17h486b30ddf29972c8E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he9cb7a45b206fee8E;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he9cb7a45b206fee8E:
	retq

	.def	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7cb5d6b883c9d05fE;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h7cb5d6b883c9d05fE:
	xorl	%eax, %eax
	retq

	.def	_ZN3asm4main17hb7cc11ea00701fdcE;
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
_ZN3asm4main17hb7cc11ea00701fdcE:
.seh_proc _ZN3asm4main17hb7cc11ea00701fdcE
	pushq	%rax
	.seh_stackalloc 8
	.seh_endprologue
	#APP

	nop

	#NO_APP
	popq	%rax
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.globl	main
	.p2align	4, 0x90
main:
.seh_proc main
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$48, %rsp
	.seh_stackalloc 48
	leaq	48(%rsp), %rbp
	.seh_setframe %rbp, 48
	.seh_endprologue
	movq	%rdx, -8(%rbp)
	movl	%ecx, -12(%rbp)
	callq	__main
	movl	-12(%rbp), %ecx
	movq	-8(%rbp), %r8
	movslq	%ecx, %rdx
	leaq	_ZN3asm4main17hb7cc11ea00701fdcE(%rip), %rcx
	xorl	%r9d, %r9d
	callq	_ZN3std2rt10lang_start17hde7798f4d69721a4E
	nop
	addq	$48, %rsp
	popq	%rbp
	retq
	.seh_endproc

	.section	.rdata,"dr"
	.p2align	3, 0x0
.L__unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h1fea5d23bba74984E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h9281da6ab201cd22E

	.section	.drectve,"yni"
	.ascii	" -exclude-symbols:_ZN3std2rt10lang_start17hde7798f4d69721a4E"
