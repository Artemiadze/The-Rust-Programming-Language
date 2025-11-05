fn main() {
    println!("Hello, world!");

    another_function();
    print_number(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    }; // <- this is 4

    println!("The value of y is: {y}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // return isn't needed, like a ';'
}