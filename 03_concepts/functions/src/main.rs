fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
    statements_and_expressions();
    function_return_values();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
    // statements perform an action but don't return a value
    // expressions evaluate to a resultant value
    //
    // statement: "let y = 5 + 6;"
    // expression: 5 + 6
    // 
    // expressions can be part of statements 
    // calling a function is an expression
    // calling macro is an expression

    // this is an expression (scope block created):
    let y = {
        let x = 3;
        x + 1 // no semicolon (if had one, would be statement and doesn't compile)
    };

    println!("The value of y is: {y}"); // 4
}

fn function_return_values() {
    // return value synonomous with last expression in function body
    // return early in function with "return <some_value>"
    // But, will return last expression implicitly
    let x = five();
    println!("The value of x is: {x}");

    // another example
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// implicitly returns 5 (last expression)
fn five() -> i32 {
    5 // no semicolon (expression)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
