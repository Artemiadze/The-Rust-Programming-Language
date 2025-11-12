use std::io; // подключение io библиотеки из стандартной библиотеки

fn main(){
    println!("Please input your numner for Fibonacci");
    let mut number = String::new(); // Новый экземпляр String

    io::stdin()
        .read_line(&mut number)  // .read_line - получение данных из ввода и запись (&) в место хранение guess
        .expect("Failed to read line");     // read_line возвращает ok/error -> expect работает при error

    // String to unsighned int 32
    let number: u32 = match number.trim().parse() {  // trim() - удаление пробелов
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let answer = fib(number);
    println!("{answer}");

}

fn fib(number: u32) -> u32 {
    if number == 1 || number == 2{
        1
    }
    else {
        fib(number-1) + fib(number-2)
    }
}