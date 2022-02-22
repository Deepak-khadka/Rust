// Topic: if elseif and else conditon 

fn display_number(input_number: i32, display_number: i32) {
    println!("{:?} is greater than {:?}", input_number, display_number)
}

fn main() {
    let number = 10;

    if number > 11 {
       display_number(number, 11)
    }
    else if number > 5 {
        display_number(number, 5)
    }
    else {
        display_number(number, 0)
    }
}