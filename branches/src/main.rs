fn main() {
    basic();
    basic_plus();
    ternary_let();
    // run_forever_loop();
    run_loop();
    nested_loop();
    while_loop();
    while_index();
    for_loop();
    for_range_reverse();
}

fn basic() {
    println!("\nif else");

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn basic_plus() {
    println!("\nif else if else");

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

fn ternary_let() {
    println!("\nternary let");

    let condition = true;
    // this throws an error because we need to know the type at compile time
    // let number = if condition { 5 } else { "six" }

    // this is fine
    let number = if condition { 5 } else { 6 };
    println!("the value of number is {number}");
}

// fn run_forever_loop() {
//     println!("\nforever loop");

//     loop {
//         println!("again!");
//     }
// }

fn run_loop() {
    println!("\nloop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}")
}

fn nested_loop() {
    println!("\nnested loops");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loop() {
    println!("\nwhile loop");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("done");
}

fn while_index() {
    println!("\nwhile loop with array index");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("value at index {index} is {}.", a[index]);
        index += 1;
    }
}

fn for_loop() {
    println!("\nfor loop");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_reverse() {
    println!("\nfor loop with reversed range");

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("done");
}