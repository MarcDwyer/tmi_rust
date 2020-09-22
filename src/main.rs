use std::env;
mod twitch_chat;

// fn something(target: i32) -> String {
//     let mut curr: Option<i32> = Some(0);
//     loop {
//         match curr {
//             Some(n) if n == target => return format!("Target reached: {}", n),
//             Some(n) => {
//                 curr = Some(n + 1);
//                 continue;
//             }
//             None => continue,
//         }
//     }
// }
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
