use std::env;
use dotenv::dotenv;
use regex::Regex;
// use egg_mode::{
//     KeyPair,
//     Token,
//     tweet,
// };
use tweers::Twitter;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, user::User},
    prelude::*,
};
use std::collections::HashMap;

struct Handler {
    re_id: Regex,
    re_meta: Regex,
    twitter_status_url: String,
    twitter: Twitter,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let botuser = ctx.cache.current_user().await;

        if !msg.mention_everyone && msg.mentions.contains(&User::from(botuser)) {
            let target_msg: Message = match msg.referenced_message {
                Some(refmsg) => *refmsg,
                None => msg,
            };

            let content = self.re_meta.replace_all(&target_msg.content, "").trim().to_string();

            println!("{}", content);

            if content.is_empty() {
                let _ = target_msg.reply(&ctx.http, "メッセージが空です").await;
                return;
            }

            if let Some(caps) = self.re_id.captures(&content) {
                println!("  Delete tweet:");
                let id: u64 = caps["id"].parse().unwrap();
                //tweet::delete(id, &self.twitter_token).await.expect("Failed to delete.");
                let res = self.twitter.delete(&format!("tweets/{}", id), HashMap::new()).await.unwrap();
                println!("{:?}", res);

                let _ = target_msg.react(&ctx.http, '❌').await;
            } else {
                println!("  Draft tweet:");
                // let tweet = tweet::DraftTweet::new(content)
                //     .send(&self.twitter_token)
                //     .await.expect("Failed to draft.");
                let mut params = HashMap::new();
                params.insert("text", content.as_str());
                let res = self.twitter.post("tweets", params).await.unwrap();
                println!("{:?}", res);
                let id = res["data"]["id"].as_str().unwrap();
            
                let _ = target_msg.reply(&ctx.http, format!("<{}/{}>", &self.twitter_status_url, id)).await;

                let _ = target_msg.react(&ctx.http, '💬').await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let twitter_account = env::var("TWITTER_ACCOUNT").expect("Expected a token in the environment");
    let consumer_key = env::var("CK").expect("Not found: CK");
    let consumer_secret = env::var("CS").expect("Not found: CS");
    let access_token = env::var("AT").expect("Not found: AT");
    let access_token_secret = env::var("AS").expect("Not found: AS");

    let twitter_status_url = format!("https://twitter.com/{}/status", twitter_account);

    let re_id = Regex::new(&format!(r"^<?{}/(?P<id>\d+)?>", twitter_status_url)).unwrap();
    let re_meta = Regex::new(r"<(@|@!|#)\d+>").unwrap();

    // let twitter_token = Token::Access {
    //     consumer: KeyPair::new(consumer_key, consumer_secret),
    //     access: KeyPair::new(access_token, access_token_secret),
    // };

    let twitter = Twitter::new(
        consumer_key,
        consumer_secret,
        access_token,
        access_token_secret
    );

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler {
            re_id, re_meta, twitter_status_url, twitter
        })
        .await.expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
