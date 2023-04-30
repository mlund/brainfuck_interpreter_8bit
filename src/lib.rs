//! This crate provides a simple function to parse brainfuck String to human readable(ASCII characters)

use error::BrainFuckError;
use unit::Unit;

/// Function to parse input to human readable(ascii characters) String
pub fn brainfuck_to_string(
    source_string: impl ToString,
    inputs: Option<Vec<char>>,
) -> Result<String, BrainFuckError> {
    let source_chars: Vec<char> = source_string.to_string().chars().collect();
    let mut unit_vec: Vec<Unit> = vec![Unit::new(0)];
    let mut pointer = 0;
    let mut result: String = String::new();

    let mut index = 0;
    let mut previous_loop_start_index = 0;

    let mut inner_inputs = Vec::new();

    if source_chars.contains(&',') {
        inner_inputs = match inputs {
            Some(v) => v,
            None => {
                #[cfg(not(feature = "ignore-input-error"))]
                return Err(BrainFuckError::InputNotProvidedError);

                #[cfg(feature = "ignore-input-error")]
                vec![]
            }
        };
    }

    while index < source_chars.len() {
        match source_chars[index] {
            '+' => {
                unit_vec[pointer] += 1;
            }
            '-' => {
                unit_vec[pointer] -= 1;
            }
            '>' => {
                pointer += 1;
                if unit_vec.len() - 1 < pointer {
                    unit_vec.push(Unit::new(0));
                }
            }
            '<' => {
                pointer -= 1;
            }
            '.' => {
                result.push(unit_vec[pointer].get_char());
            }
            '[' => {
                if unit_vec[pointer].get_raw() == 0 {
                    let loop_closed_index = source_chars[index..].iter().position(|&x| x == ']');
                    index = match loop_closed_index {
                        Some(v) => v,
                        None => return Err(BrainFuckError::LoopNotClosedError(index)),
                    };
                }
                previous_loop_start_index = index;
            }
            ']' => {
                if unit_vec[pointer].get_raw() != 0 {
                    index = previous_loop_start_index - 1;
                }
            }
            ',' => {
                match inner_inputs.first() {
                    Some(v) => {
                        unit_vec[pointer] = Unit::new_from_char(v);
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

mod error;
mod unit;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hello_world() {
        let brain_fuck_string = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.".to_string();
        let readable_string = brainfuck_to_string(&brain_fuck_string, None).unwrap();
        assert_eq!(readable_string, "Hello World!");
    }

    #[test]
    fn simple() {
        let brain_fuck_string = ">>><<+++++++++++++++++++++++++++++++++.++.--.";
        let readable_string = brainfuck_to_string(&brain_fuck_string, None).unwrap();
        assert_eq!(readable_string, "!#!");
    }

    #[test]
    fn input() {
        let brain_fuck_string = ",>,.<.";
        let input = vec!['a', 'b'];
        let readable_string = brainfuck_to_string(&brain_fuck_string, Some(input)).unwrap();
        assert_eq!(readable_string, "ba");
    }
}
