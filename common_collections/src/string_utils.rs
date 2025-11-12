pub fn study_string(){
    // creating empty string
    let mut mut_string = String::new();

    //using to_string for creating string
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    // let s = "initial contents".to_string();
    println!("{}", s);
    //or
    let new_s = String::from("initial contents 2");
    println!("{}", new_s);

    //updating string
    println!("Old version of mut_string is {mut_string}");
    mut_string.push_str(", update!");
    println!("Updated version of mut_string is {mut_string}");
    mut_string.push('1'); // push add to string 1 symbol
    println!("Updated version of mut_string due to push is {mut_string}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    //Concatenation
    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    let s = str1.clone() + "-" + &str2 + "-" + &str3;
    println!("{s}");

    //OR
    let s = format!("{str1}-{str2}-{str3}");
    println!("{s}");

    //index
    let str_hi = String::from("hi");
    //let h = s1[0]; <- it doesn't work
    let h: &str = &str_hi[0..1]; // use only slicing
    println!("str_hi[0] is {h}");


    //iteration over string
    for c in "String".chars() {
        println!("{c}");
    }
}