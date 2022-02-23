use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = &args[1];

    let re = Regex::new(r"^https://twitter.com/scienceboy_jp/status/(?P<id>\d+)").unwrap();

    if let Some(caps) = re.captures(url) {
        println!("ID of '{}' is {}", url, &caps["id"]);
    }
}
