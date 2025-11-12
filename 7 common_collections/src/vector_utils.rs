pub fn study_vector(){
    //creating vector
    let mut v: Vec<i32> = Vec::new();
    let numbers = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element in vector 'v' is {third}");

    //Getting element from vector
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //Iteration
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("new {i}");
    }

    //new vector
    println!("Numbers vector:");
    for i in &numbers {
        println!("{i}");
    }
}