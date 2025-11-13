mod box_t;

use std::rc::Rc;
use crate::box_t::Example;

fn main() {
    box_t::box_example();

    let m = box_t::MyBox::new(String::from("Rust"));
    hello(&m);
    println!("CustomSmartPointer created.");
    drop(m);
    println!("CustomSmartPointer dropped before the end of main.");

    println!("--------------");

    //The same object (Example) can belong to several owners (x, y).
    //Rust guarantees that the object will be deleted only when all owners disappear.
    let x = Rc::new(Example);    
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);    
    println!("C");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}