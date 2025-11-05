fn main(){
    // floating point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    //boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}