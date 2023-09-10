# Designing Q

1. Tiny core, everything else is macros
2. Actor-based concurrency
3. support mutable data within functions, but can't share data mutably across processes
3. Tiny VM, runs on WebAssembly
3. No exceptions, errors are value

It should feel quite familiar to people writing Rust, Erlang, Elixir, ReScript, and OCaml.

## Primitive Values

```rust
// Unit value
()

// A list
[a,b,c]

// A tuple
(1,2,3)

// A string
`hello world`
```

## Module system

We want a strict namespacing that follows the filesystem so that navigating files

```rust
// files are modules.
// file: math.q
square(x) { x * x }

// files in a subfolder are submodules
// file: math/nat.q 
enum Nat { Zero, Succ(Nat) }
zero()  -> Zero

next(Zero) { Succ(Zero) }
next(n) { Succ(n) }

// file: math.q
mod nat
one() { nat:next(nat:zero()) }
```

## New Types

```rust
@derive(Debug, Serializer, Deserializer)
struct User {
    name: String
}

@derive(Debug)
enum Role {
    Admin,
}


```

## Primitive Operations

```rust
match expr { pattern => expr2 }
spawn { $body }
receive { pattern => expr } after expr { expr }
```

## Named Values and Variables

All primitive values are by default immutable.

```rust
// defines a _value_ named x that can't be changed
// but can be rebound
x = 1 
x = x + 1
x == 2

// defines a _variable_ named y that can be changed
mut y = 1
y <- y + 2 // this is the exact same y as before
y == 2
```

## Actor-based concurrency

```rust
hello_proc = spawn loop {
  receive {
    SayHello(name) => {
      std.io.print("Hello {name}\n")
    }
  } after 1000ms {
    std.io.print("Noone to say hi to :(")
  }
}

msg = SayHello("captain")
hello_proc.send(msg)

mut msg = SayHello("captain")
hello_proc.send(msg) // <-- compilation error! `msg` must not be `mut`
```

## Loop is a macro

`loop { body }` desugars to:

```rust
loop(state) {
  match $body {
    Control:Continue(state2) => loop(state2)
    Control:Break(state2) => state2
  }
}
```

so the example above


```rust
hello_proc = spawn loop {
  receive {
    SayHello(name) => {
      std.io.print("Hello {name}\n")
    }
  } after 1000ms {
    std.io.print("Noone to say hi to :(")
    break;
  }
}
```

desugars to

```rust
hello_proc_loop() {
  next = receive {
    SayHello(name) => {
      std.io.print("Hello {name}\n")
      Control:Continue()
    }
  } after 1000ms {
    std.io.print("Noone to say hi to :(")
    Control:Break()
  }

  match next {
      Control:Continue() => hello_proc_loop()
      Control:Breka() => ()
  }
}
```

