#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::string::ToString;
use brainfuck_interpreter::brainfuck_to_string;
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let brain_fuck_string = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.";
    let readable_string = brainfuck_to_string(&brain_fuck_string, None).unwrap();
    println!("{}", readable_string.as_str());
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    println!("!");
    loop {}
}
