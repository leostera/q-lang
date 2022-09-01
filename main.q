foo = (A, B) { print(A) }
  call_ext print r0

main = (Arg) { foo(Arg, Arg) }
  move r0 r1 
  call l1
