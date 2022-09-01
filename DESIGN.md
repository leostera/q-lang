# Designing Q

1. Tiny core, everything else is macros
2. Tiny VM, that runs on WebAssembly

## Primitive Values

```rust
// A list
[a,b,c]

// A tuple
<1,2,3>

// A string
`hello world`
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
