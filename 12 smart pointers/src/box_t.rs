use std::ops::Deref;

pub fn box_example(){
    // Boxes allow you to store data on the heap rather than the stack.
    let b = Box::new(5);
    println!("b = {b}");

    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);
}

// Defining Our Own Smart Pointer
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Creating '*' for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

//Rc<T>

pub struct Example;

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}