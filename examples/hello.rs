#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use brainfuck_interpreter::interpret;
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let input = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.";
    let output = interpret(&input.as_bytes(), None).unwrap();
    println!("Input: {}", input);
    println!("Output: {}", output.as_str());
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    println!("!");
    loop {}
}
