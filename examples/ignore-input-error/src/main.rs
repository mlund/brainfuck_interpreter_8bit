use brainfuck_interpreter::brainfuck_to_string;

fn main() {
    // ignore input not provided
    let brain_fuck_string = ",,+++++++++++++++++++++++++++++++++.";
    let readable_string = brainfuck_to_string(&brain_fuck_string, None).unwrap();
    println!("{readable_string}");

    // ignore input not enough
    let brain_fuck_string = ",.,.";
    let input = vec!['a'];
    let readable_string = brainfuck_to_string(&brain_fuck_string, Some(input)).unwrap();
    println!("{readable_string}");
}
