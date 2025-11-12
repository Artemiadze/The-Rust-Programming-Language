#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Функция: возвращает ценность монеты
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // пример 1: match — полный разбор enum
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Функция: демонстрация if let
fn check_ip_version(addr: &IpAddr) {
    // пример 2: if let — проверяем только один вариант enum
    if let IpAddr::V4(a, b, c, d) = addr {
        println!("This is IPv4 address: {}.{}.{}.{}", a, b, c, d);
    } else {
        println!("This is not IPv4 (probably IPv6).");
    }
}

// Функция: демонстрация let else
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn safe_division_example(a: i32, b: i32) {
    // пример 3: let else — извлекаем значение и выходим, если None
    let Some(result) = divide(a, b) else {
        println!("Cannot divide by zero!");
        return;
    };

    println!("{a} / {b} = {result}");
}

fn main() {
    // match demo
    let my_coin = Coin::Dime;
    let cents = value_in_cents(my_coin);
    println!("The coin is worth {cents} cents.");

    // if let demo
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    check_ip_version(&home);
    check_ip_version(&loopback);

    // let else demo
    safe_division_example(10, 2);
    safe_division_example(8, 0);
}
