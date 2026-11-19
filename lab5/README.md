# Lab 5

A small language built on two traits: `Expr` (an expression returns a `u64`) and `Stmt` (a statement returns nothing).

It includes printing, an empty statement, a sequence of two statements (with simplifications when one of them is empty), a conditional, repetition with the iteration count in the type, reading a constant from a context, reading an outside variable, writing to a variable, and a form that swaps a context value for the duration of the computation.

Covers traits, generics, const generics, lifetimes, and tests of nested programs.

```
cargo test
cargo run
```
