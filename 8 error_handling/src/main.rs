use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Create the file if it is not found
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            }, // If file is found but there is another error, panic
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
    // It call enum Result<T, E> 

    let second_greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/