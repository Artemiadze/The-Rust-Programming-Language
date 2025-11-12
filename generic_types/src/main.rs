mod math_utils;
mod point;
mod aggregator;

use aggregator::{SocialPost, Summary, NewsArticle};

fn main() {
    // i32 type
    let number_list = vec![34, 50, 25, 100, 65];

    let result = math_utils::largest_int(&number_list);
    println!("The largest number is {result}");


    // char type
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = math_utils::largest_char(&char_list);
    println!("The largest char is {result}");

    // using generic function
    let result = math_utils::largest(&number_list);
    println!("The largest number is {result}");

    let result = math_utils::largest(&char_list);
    println!("The largest char is {result}");

    // using generic struct and method
    use crate::point::Point;
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //use media aggregator library
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize()); 

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
