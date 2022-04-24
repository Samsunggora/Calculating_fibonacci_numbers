use std::io;

fn main() {
    println!("Please enter fib number:");

    let mut user_number = String::new();
    io::stdin()
        .read_line(&mut user_number)
        .expect("Failed to read line");

    let user_number_int: i128 = user_number.trim().parse().ok().expect("Input err!");

    let mut fib_num_1: i128 = 1;
    let mut fib_num_2: i128 = 1;
    let mut i: i128 = 0;
    while i < (user_number_int - 2 as i128) {
        let mut fib_sum = fib_num_1 + fib_num_2;
        fib_num_1 = fib_num_2;
        fib_num_2 = fib_sum;
        i += 1 as i128;
        println!("{fib_sum}");
    }

}
