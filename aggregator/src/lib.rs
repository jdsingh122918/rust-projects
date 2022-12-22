pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn feature() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    #[ignore = "Testing out"]
    fn expensive_test() {
        todo!()
    }
}
