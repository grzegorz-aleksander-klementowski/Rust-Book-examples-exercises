// That's the „interface”. Every type should have it's own implementation
pub trait Summary {
    // Just signature
    //fn summary(&self) -> String;
    fn summary_author(&self) -> String;

    // The default implementation of the method
    fn summary(&self) -> String {
        format!("Read more from {}…", self.summary_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewArticle {
    pub fn new(headline: &str, location: &str, author: &str, content: &str) -> Self {
        Self {
            headline: headline.to_string(),
            location: location.to_string(),
            author: author.to_string(),
            content: content.to_string(),
        }
    }
}

// The default implementation of `NewArticle`
impl Summary for NewArticle {
    fn summary_author(&self) -> String {
        self.author.to_string()
    }
}

// The specified implementation
/* impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
} */

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl SocialPost {
    pub fn new(username: &str, content: &str, reply: bool, repost: bool) -> Self {
        Self {
            username: username.to_string(),
            content: content.to_string(),
            reply,
            repost,
        }
    }
}

impl Summary for SocialPost {
    // hiden summary function to run default trait method
    /* fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    } */

    fn summary_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// ---------------TRAIT BOUNDS---------------- \\

// It's a shorter syntax of the trait bound syntax
/* pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summary());
} */

//the trait bound syntax
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news: {}", item.summary());
}
