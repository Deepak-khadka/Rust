// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, display the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut number = 5;

    while number >=1 {
        println!("Count down : {:?} ", number);
        number = number - 1;
    }
    println!("done!")
}