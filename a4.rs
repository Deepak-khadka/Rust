// Topic: fundamentals of match
//
// * add logic to program
// * Similar to if--else
// match vs else..if
// * match will be checked by the compiler
//  - If a new possibility is added, you will be notified when this occurs
// * else..if is not checked by the compiler
//  - if a new possibility is added, your code may contain a bug

fn main() {
    
    let bool_value = true;
    let some_num = 3;
    let name = "deepak";


    match bool_value {
        true => println!("Here is true value"),
        false => println!("Here is false value")
    }

    // _ this is else condition if no any other value is match the line no 22 will print
    match some_num {
        1 => println!("Value is 1"),
        2 => println!("Value is 2"),
        3 => println!("Value is 3"),
        _ => println!("Value is null")
    }

    match name {
         "deepak" => println!("It's me"),
         "ramesh" => println!("Hey, ramesh how are you"),
        _ => println!("Nice to meet you pal")
    }

}