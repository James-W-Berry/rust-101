fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    expression();

    let five_test = give_me_five();
    println!("the value of give me five is {five_test}.");

    let five_test_plus_one = plus_one(five_test);
    println!("the value of {five_test} plus 1 is {five_test_plus_one}.");
}

fn another_function(x: i32, unit: char) {
    println!("the measurement is: {x}{unit}.");
}

fn expression() {
 let y = {
    let x = 3;
    x + 1
 };

 println!("y evaluates to {y}.");
}

fn give_me_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}