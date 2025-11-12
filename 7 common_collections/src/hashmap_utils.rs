use std::collections::HashMap;

pub fn study_hashMap(){
    //Creating empty hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Iteration
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //getting value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score for team {team_name} is {score}");

    //Overwriting the value
    scores.insert(String::from("Red"), 11);
    scores.insert(String::from("Red"), 21);

    // Adding a Key and Value Only If a Key Isn’t Present

    scores.entry(String::from("Brown")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100); // it won't be inserted

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}