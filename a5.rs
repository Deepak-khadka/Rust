// Loop 

fn main() {
    let mut loop_iteration = 3;

    loop {
        println!("Countdoun : {:?}", loop_iteration);
        loop_iteration = loop_iteration - 1;

        if loop_iteration == 0 {
            break;
        } 
    }
    println!("done!")
}