# Mon Aug 22 21:12:01 CEST 2022

-[x] be able to distinguish between a good EOF and an interrupted parser due to an EOF
-[ ] get rid of diagnostic 
-[ ] skip ahead on a parse error

# Thu Aug 18 20:13:16 CEST 2022

-[x] begin our parser
-[x] executable
-[x] play around with diagnostics
-[x] explore parse trait 

For next time:
* how do we ensure that all expressions get their diagnostic in,
  even when we are out of a file (reached EOF)?

* what is the distinction between parsing errors and diagnostics

# Wed Aug 17 21:48:47 CEST 2022

-[X] set up a repo that can build
-[X] begin a grammar

### Notes

1. untyped macro expansion
2. <...> untyped language
3. type checker
4. typed macro expansion


convenient
data is always immutable
first-class concurrency

str = "hello world"

hello_world = () x{
  print(str)
}

PI = 3.14

---
[items, diagnostics]



@cli
type app = {
  verbose: bool
}


main(args) {
  spawn hello_world()
}

System.Module.all()


