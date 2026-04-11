// That's the „interface”. Every type should have it's own implementation
pub trait Summary {
    fn summary(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
