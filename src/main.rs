use std::env;
mod parser;
mod twitch_chat;

fn main() {
    let client_id = env::var("client_id").unwrap();
    let oauth = env::var("oauth").unwrap();
    println!("{}, {}", client_id, oauth);
    let creds = twitch_chat::Credentials {
        client_id,
        oauth,
        user_name: String::from("roy_stang"),
    };

    let tc = twitch_chat::TwitchChat::new(creds);
    match tc.connect() {
        Ok(v) => println!("{}", v),
        Err(e) => panic!(format!("{:?}", e)),
    };
}
