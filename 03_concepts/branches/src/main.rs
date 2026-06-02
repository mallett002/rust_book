fn main() {
    let number = 7;

    // if number < 5 {
    if number != 0 {
       println!("Condition was true");
    } else {
       println!("Condition was false");
    }

    else_if();
    if_is_expression();
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
       println!("number is divisible by 4");
    } else if number % 3 == 0 {
       println!("number is divisible by 3");
    } else if number % 2 == 0 {
       println!("number is divisible by 2");
    } else {
       println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_is_expression() {
    // `if` is an expression (evaluates to a boolean)
    // can use it on right side of `let` statement to assign to var
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // won't compile: need to be same type (the if expression needs to result in 1 type)
    // let number = if condition { 5 } else { "six" };
}
