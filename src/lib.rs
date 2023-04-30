use unit::Unit;
pub fn brainfuck_to_string(source_string: impl ToString, inputs: Option<Vec<char>>) -> String {
    let source_chars: Vec<char> = source_string.to_string().chars().collect();
    let mut unit_vec: Vec<Unit> = vec![Unit::new(0)];
    let mut pointer = 0;
    let mut result: String = String::new();

    let mut index = 0;
    let mut previous_loop_start_index = 0;

    let mut inner_inputs = Vec::new();

    if source_chars.contains(&',') {
        inner_inputs = inputs.expect("need inputs");
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
                    let a = source_chars[index..]
                        .iter()
                        .position(|&x| x == ']')
                        .expect("can't find ]");
                    index = a;
                }
                previous_loop_start_index = index;
            }
            ']' => {
                if unit_vec[pointer].get_raw() != 0 {
                    index = previous_loop_start_index - 1;
                }
            }
            ',' => {
                let f = inner_inputs.first().expect("input not enough").clone();
                unit_vec[pointer] = Unit::new_from_char(&f);
                inner_inputs.remove(0);
            }
            _ => {}
        }
        if index == source_chars.len() - 1 {
            break;
        }
        index += 1;
    }

    result
}

mod unit;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hello_world() {
        let brain_fuck_string = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.".to_string();
        let readable_string = brainfuck_to_string(&brain_fuck_string, None);
        assert_eq!(readable_string, "Hello World!");
    }

    #[test]
    fn simple() {
        let brain_fuck_string = ">>><<+++++++++++++++++++++++++++++++++.++.--.";
        let readable_string = brainfuck_to_string(&brain_fuck_string, None);
        assert_eq!(readable_string, "!#!");
    }

    #[test]
    fn input() {
        let brain_fuck_string = ",>,.<.";
        let input = vec!['a', 'b'];
        let readable_string = brainfuck_to_string(&brain_fuck_string, Some(input));
        assert_eq!(readable_string, "ba");
    }
}
