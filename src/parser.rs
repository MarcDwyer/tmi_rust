use std::collections::HashMap;

pub enum Commands {
    PRIVMSG,
    ROOMSTATE,
    CLEARCHAT,
    CLEARMSG,
    GLOBALUSERSTATE,
    USERNOTICE,
    USERSTATE,
    JOIN,
    NONE,
    PING,
    WHISPER,
    NOTICE,
    ZERO,
}

pub struct TwitchMessage {
    raw: String,
    tags: HashMap<String, String>,
    prefix: String,
    commands: Commands,
    channel: String,
    params: Vec<String>,
    message: String,
    username: String,
}
impl TwitchMessage {
    pub fn new() -> Self {
        let t_msg = TwitchMessage {
            raw: String::from(""),
            tags: HashMap::new(),
            prefix: String::from(""),
            commands: Commands::ZERO,
            channel: String::from(""),
            params: Vec::new(),
            message: String::from(""),
            username: String::from(""),
        };
        t_msg
    }
}
pub fn parcer(data: String) -> Option<TwitchMessage> {
    let mut position: usize = 0;
    let mut next_space: usize = 0;
    let mut t_msg = TwitchMessage::new();
    let ch = data.char_indices();
    let bytes = data.as_bytes();
    if bytes[0] == 64 {
        next_space = match data.find(" ") {
            Some(i) => i,
            None => return None,
        };

        let raw_tags = &data[1..next_space];
        let tags = raw_tags.split(";");
        for tag in tags {
            let kv = tag.split("=");
            // let (k, v) = (kv[0], kv[1]);
        }
    }
    // bytes[0]
    None
}
