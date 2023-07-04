//! This crate provides a simple function to parse brainfuck String to human readable(ASCII characters)

#![no_std]
#![feature(error_in_core)]

extern crate alloc;
extern crate mos_alloc;
extern crate ufmt_stdio;

pub mod error;
pub mod unit;

//use core::option::{Option, Option::Some};
use alloc::string::String;
use alloc::{vec, vec::Vec};
use error::BrainFuckError;
use unit::Unit;

/// Function to parse input to human readable(ascii characters) String
pub fn brainfuck_to_string(
    source_chars: &str,
    inputs: Option<Vec<char>>,
) -> Result<String, BrainFuckError> {
    let mut unit_vec: Vec<Unit> = vec![Unit::default()];
    let mut pointer = 0;
    let mut result = String::new();
    let mut index = 0;
    let mut previous_loop_start_index = 0;
    let mut inner_inputs = Vec::new();

    if source_chars.contains(',') {
        inner_inputs = match inputs {
            Some(v) => v,
            None => {
                #[cfg(not(feature = "ignore-input-error"))]
                return Err(BrainFuckError::InputNotProvidedError);

                #[cfg(feature = "ignore-input-error")]
                Vec::new()
            }
        };
    }

    while index < source_chars.len() {
        match source_chars.as_bytes()[index] {
            b'+' => unit_vec[pointer] += 1,
            b'-' => unit_vec[pointer] -= 1,
            b'>' => {
                pointer += 1;
                if unit_vec.len() - 1 < pointer {
                    unit_vec.push(Unit::default());
                }
            }
            b'<' => pointer -= 1,
            b'.' => result.push(unit_vec[pointer].into()),
            b'[' => {
                if u8::from(unit_vec[pointer]) == 0 {
                    let loop_closed_index = source_chars[index..].chars().position(|x| x == ']');
                    index = match loop_closed_index {
                        Some(v) => v,
                        None => return Err(BrainFuckError::LoopNotClosedError(index)),
                    };
                }
                previous_loop_start_index = index;
            }
            b']' => {
                if u8::from(unit_vec[pointer]) != 0 {
                    index = previous_loop_start_index - 1;
                }
            }
            b',' => {
                match inner_inputs.first().copied() {
                    Some(v) => {
                        unit_vec[pointer] = v.into();
                        inner_inputs.remove(0);
                    }
                    None =>
                    {
                        #[cfg(not(feature = "ignore-input-error"))]
                        return Err(BrainFuckError::InputNotEnoughError)
                    }
                };
            }
            _ => {}
        }
        if index == source_chars.len() - 1 {
            break;
        }
        index += 1;
    }
    Ok(result)
}
