use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn example_channels(){
    let (tx, rx) = mpsc::channel(); // create new channel
    // transmitter - send data, receiver - receive data

    let tx1 = tx.clone(); // second transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // send value through channel
            thread::sleep(Duration::from_secs(1));
        }
    });

    // another thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 2 thread work simultaneously, so we use a loop to receive values as they come

    for received in rx {
        println!("Got: {received}");
    }

}