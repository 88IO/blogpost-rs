use egg_mode::tweet::DraftTweet;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let consumer_key = env::var("CK").expect("Not found: CK");
    let consumer_secret = env::var("CS").expect("Not found: CS");
    let access_token = env::var("AT").expect("Not found: AT");
    let access_token_secret = env::var("AS").expect("Not found: AS");

    let consumer = egg_mode::KeyPair::new(
        consumer_key, consumer_secret
    );
    let access = egg_mode::KeyPair::new(
        access_token, access_token_secret
    );

    let token = egg_mode::Token::Access {
        consumer, access
    };

    let tweet = DraftTweet::new("Test tweet from egg_mode.");
    let _ = tweet.send(&token).await;
}
