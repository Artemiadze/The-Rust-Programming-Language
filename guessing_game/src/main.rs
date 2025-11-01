use std::cmp::Ordering; // Варианты сравнения из стд библиотеки
use std::io; // подключение io библиотеки из стандартной библиотеки

use rand::Rng;

// обьявление функции
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // бесконечный цикл
    loop {
        println!("Please input your guess.");

        // let - создание переменной, mut - изменяемой перем.
        let mut guess = String::new(); // Новый экземпляр String

        io::stdin()
            .read_line(&mut guess)  // .read_line - получение данных из ввода и запись (&) в место хранение guess
            .expect("Failed to read line");     // read_line возвращает ok/error -> expect работает при error

        // String to unsighned int 32
        let guess: u32 = match guess.trim().parse() {  // trim() - удаление пробелов
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}
