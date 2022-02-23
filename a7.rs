enum Color {
    RED,
    BLUE,
    BLACK
}

fn check_color(my_color: Color) {
    match my_color {
        Color::RED => println!("Here is red color"),
        Color::BLUE => println!("Here is blue color"),
        Color::BLACK => println!("Here is black color")
    }
}

fn main() {
    check_color(Color::BLACK);
}