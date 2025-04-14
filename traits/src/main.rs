use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("Author name is protected")
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

// Traits as parameters
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple traits as parameters
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!(
//         "Breaking news! {} and {}",
//         item1.summarize(),
//         item2.summarize()
//     );
// }

// If we want both item to be of the same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     println!(
//         "Breaking news! {} and {}",
//         item1.summarize(),
//         item2.summarize()
//     );
// }

// Mutilple traits in one type
// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {
//     println!(
//         "Breaking news! {} and {}",
//         item1.summarize(),
//         item2.summarize()
//     );
// }

// pub fn notify(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
// }

// Using where clause to clear function signature
// pub fn notify<T, U>(item: &T, item2: &U)
// where
//     T: Summary,
//     U: Display,
// {
//     println!("Breaking news! {}", item.summarize());
// }

fn return_summarizable() -> impl Summary {
    Tweet {
        author: String::from("Jane Doe"),
        content: String::from("This is a tweet."),
        reply: false,
        retweet: false,
    }
}

// Using generics to create a struct
// Generics allow us to create a struct that can hold any type
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    fn cmp(&self) {
        if self.x < self.y {
            println!("x is less than y");
        } else if self.x > self.y {
            println!("x is greater than y");
        } else {
            println!("x is equal to y");
        }
    }
}

// Generics with traits
// Generics allow us to create a function that can take any type
// that implements a trait
// This allows us to create a function that can take any type
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         format!("{}", self)
//     }
// }

fn main() {
    let news_article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Breaking News"),
        content: String::from("This is the content of the news article."),
    };

    let tweet = Tweet {
        author: String::from("Jane Doe"),
        content: String::from("This is a tweet."),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.summarize_author());
    println!("{}", news_article.summarize());
    println!("{}", news_article.summarize_author());

    notify(&news_article);

    return_summarizable().summarize();
}
