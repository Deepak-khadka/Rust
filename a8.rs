/*
  Topic: Organizing similar data using structs

  Requirements:
  * Print the flavor of a drink and it's fluid ounces

  Notes:
  * Use an enum to create different flavors of drinks
  * Use a struct to store drink flavor and fluid ounce information
  * Use a function to print out the drink flavor and ounces
  * Use a match expression to print the drink flavor
*/

// * Use an enum to create different flavors of drinks
enum Flavor {
    CocaCola,
    Pepsi,
    Mirinda
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

//  * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {

    //  * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::CocaCola => println!("Flavor: Coca cola"),
        Flavor::Pepsi => println!("Flavor: Pepsi"),
        Flavor::Mirinda => println!("Flavor: Mirinda"),
    }

    println!("oz: {:?}", drink.fluid_oz)

}
fn main() {
 let neputer = Drink {
     flavor: Flavor::CocaCola,
     fluid_oz: 9.55
 };

 print_drink(neputer);

}