use std::thread;

pub fn closure_example() {
    let add_one = |x| x + 1;
    println!("{}", add_one(5)); // just like 'lambda' in Python
    println!("------------------------------");

    let factor = 2;
    let multiply = |x| x * factor; // factor — from the surrounding scope
    println!("{}", multiply(5)); // but after using closure you cannot change 'factor'
    println!("------------------------------");

    //In this example, the closure 'add_suffix' takes a mutable reference to a String and appends " world" to it.
    let mut s = String::from("Hello");
    let add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s}");
    add_suffix(&mut s);
    println!("------------------------------");

    let numbers = vec![1, 2, 3];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
    println!("------------------------------");

    //work with immutable references
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    println!("------------------------------");

    //work with mutable references
    let mut list_2 = vec![2, 2, 2];
    println!("Before defining closure: {list_2:?}");
    let mut borrows_mutably = || list_2.push(7);
    borrows_mutably();
    println!("After calling closure: {list_2:?}");
    println!("------------------------------");

    //threads
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    println!("------------------------------");
}