# 8-bit brainfuck interpreter

This is an 8-bit port of the crate
[`brainfuck_interpreter`](https://github.com/BryantTseng/brainfuck_interpreter)
by BryantTseng.
This requires [`rust-mos`](https://llvm-mos.org/wiki/Rust) and
to run using the vanilla simulator from the llvm-mos-sdk, do:

~~~ bash
cargo run --release --example hello
~~~

You may also target specific 8-bit micro-computers and
then run using an emulate after building:

~~~ bash
cargo build --release --target mos-c64-none --example hello
~~~
