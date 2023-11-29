.global _start
.align 2 // on raspberry pi it might be 4; on 64bit binary on apple silicon hence 2

_start:
  b _printf
  b _terminate

_printf:
  mov X0, #1    // stdout
  adr X1, helloworld    // hello world address
  mov X2, #12   // length of hello world string
  mov X16, #4   // write to stdout
  svc 0         // syscall

_terminate:
  mov X0, #0    // return 0
  mov X16, #1   // terminate 
  // svc is a supervisor call; it allows the application to call the OS
  svc 0         // syscall


helloworld: .ascii "hello world\n"
