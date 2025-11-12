fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // or let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {x}, also x is {0}", tup.0);
    println!("The value of y is: {y}, also y is {0}", tup.1);
    println!("The value of z is: {z}, also z is {0}", tup.2);

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("The value of x is: ({}, {})", x.0, x.1);

    //array
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // name: [type; length]
    let a = [3; 5];     // same as let a = [3, 3, 3, 3, 3];

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]); 
}