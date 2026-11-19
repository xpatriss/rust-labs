# Lab 4

Three separate pieces:

- `divisors` returns the divisors of a positive integer as a `BTreeSet` of `NonZero<u32>`, with a short timing measurement,
- `assert_sorted` panics when a `&[u32]` is not sorted in ascending order,
- a TCP server on `127.0.0.1:7878`: the client sends a path length (4 bytes, big-endian) and the path itself, and the server replies with the file names in that directory.

Covers collections, `NonZero`, I/O errors, reading and writing a stream in chunks, and tests (including network tests on a random port).

`cargo run` starts the server and does not exit on its own. The tests do not depend on that process:

```
cargo test
```
