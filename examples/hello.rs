#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use brainfuck_interpreter::brainfuck_to_string;
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let input = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.";
    let output = brainfuck_to_string(&input, None).unwrap();
    println!("Input: {}", input);
    println!("Output: {}", output.as_str());
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    println!("!");
    loop {}
}
