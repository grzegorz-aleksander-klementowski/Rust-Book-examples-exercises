use aggregator::{Summary, notify, returns_summarizable};

pub mod aggregator;

fn main() {
    let apply_to_cybersecurity_post = aggregator::SocialPost::new(
        "g_klem",
        "My new start in Cybersecurity. I just applied to cybersecurity course. Can't wait to pass ISC2!",
        false,
        false,
    );

    println!("1 new post: {}", apply_to_cybersecurity_post.summary());

    println!("2 new post: {}", returns_summarizable().summary());

    let cybersecurity_article = aggregator::NewArticle::new(
        "My way to improve ybersecurity skills",
        "Pławna Dolna, Lower Silesia, The Commonwealth of Poland",
        "Grzegorz Aleksander Klementowski",
        "When I find out that skills of cybersecurity is necessary today, not option, I decided to sing up into cybersecurity course, to improve my skills.",
    );

    println!("New article avaible! {}", cybersecurity_article.summary());
    notify(&cybersecurity_article, &cybersecurity_article);
}
