fn main() {
    // loop {
    //     println!("Hello, world!");
    // }

    returning_values_from_loops();
    loop_labels();
}

fn returning_values_from_loops() {
    let mut counter = 0;

    // save result of loop to value
    let result = loop {
        counter += 1;        

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // "break" exits loop and saves to var
    // "return" could be used too, but would exit the whole function
}

fn loop_labels() {

}
