fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let mut y = 6;
    println!("The value of y is: {y}");

    y = 22;
    println!("The new value of y is: {y}");

    // constants can only be assigned a constant expression, but it can be (1+2 and not x + y)
    // const can be globally declared
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The  value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); - mistake

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
