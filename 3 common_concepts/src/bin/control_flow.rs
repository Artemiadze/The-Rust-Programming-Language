fn main() {
    let mut number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }


    number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let in IF
    let condition = true;
    number = if condition { 5 } else { 6 }; // all conditions should be the same types

    println!("The value of number is: {number}");

    //  infinite cycle - loop
    /*loop {
        println!("again!");
    }*/

    // returns value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    cycle();

    number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    //while
    the_same();
}

fn cycle(){
    /*Внешний цикл имеет метку 'counting_up и будет считать от 0 до 2. 
    Внутренний цикл без метки будет считать от 10 до 9. 
    Первый break без указания метки завершит только внутренний цикл. 
    Оператор break 'counting_up; завершит внешний цикл*/
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

fn the_same(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }
}
