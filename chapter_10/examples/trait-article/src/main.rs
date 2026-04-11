use aggregator::Summary;

pub mod aggregator;

fn main() {
    let apply_to_cybersecurity_post = aggregator::SocialPost::new(
        String::from("My new start in Cybersecurity"),
        String::from("I just applied to cybersecurity course. Can't wait to pass ISC2!"),
        false,
        false,
    );

    println!("1 new post: {}", apply_to_cybersecurity_post.summary());
}
