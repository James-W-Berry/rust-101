fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing 
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {}", y);
    }
    println!("The value of y is {}", y);

    // change variable type with shadowing
    let z = "shadow test";
    println!("The value of z is {}", z);
    let z = z.len();
    println!("The value of z is {}", z);
}
