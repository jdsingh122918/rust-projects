use aggregator::{Summary, NewsArticle};

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1, 13, 420, 911, 69, 123];
    println!("Largest number is {}", largest(&number_list));

    let newsarticle = NewsArticle {
        headline: String::from("Greatest Books ever written"),
        location: String::from("Mother Center, California"),
        author: String::from("Master"),
        content: String::from("Autobiography of a Yogi"),
    };
    println!("Newsarticle --> {}", newsarticle.summarize());
}
