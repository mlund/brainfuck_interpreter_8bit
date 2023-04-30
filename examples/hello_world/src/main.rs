use brainfuck_interpreter::brainfuck_to_string;

fn main() {
    let brain_fuck_string = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.".to_string();
    let readable_string = brainfuck_to_string(&brain_fuck_string, None).unwrap();
    println!("{readable_string}")
}
