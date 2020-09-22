use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;

use irc_message::IrcMessage;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

pub struct TwitchChat {
    creds: Credentials,
    sender: Option<Sender<OwnedMessage>>,
}

pub struct Credentials {
    pub user_name: String,
    pub client_id: String,
    pub oauth: String,
}

impl TwitchChat {
    pub fn new(creds: Credentials) -> Self {
        return TwitchChat {
            creds,
            sender: None,
        };
    }
    pub fn connect(mut self) -> Result<String, RecvError> {
        // await ws.send(
        //     `PASS oauth:${this.twitchCred.oauth}`,
        //   );
        //   await ws.send(`NICK ${this.twitchCred.userName}`);
        //ws://irc-ws.chat.twitch.tv:80
        let url: &'static str = "ws://irc-ws.chat.twitch.tv:80";

        let client = match ClientBuilder::new(url).unwrap().connect_insecure() {
            Ok(c) => c,
            Err(e) => panic!(format!("Error connecting to socket: {}", e)),
        };

        let (mut receiver, mut sender) = client.split().unwrap();

        let (tx, rx) = channel();
        let tx_1 = tx.clone();
        self.sender = Some(tx);
        let (tx_main, rx_main): (Sender<String>, Receiver<String>) = channel();
        let send_loop = thread::spawn(move || {
            loop {
                // Send loop
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        // If it's a close message, just send it and then return.
                        return;
                    }
                    _ => (),
                }
                // Send the message
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });
        //PONG :tmi.twitch.tv
        let receive_loop = thread::spawn(move || {
            // Receive loop
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        // Got a close message, so send a close message and return
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        }
                    }
                    OwnedMessage::Text(msg) => {
                        println!("Raw: {}", msg);
                        let m = &msg[..];
                        let irc = match IrcMessage::parse_own(m) {
                            Some(parsed) => parsed,
                            None => continue,
                        };
                        println!("Parsed: {:?}", irc.command);
                    }
                    // Say what we received
                    _ => {
                        println!("Gamer: {:?}", message);
                    }
                }
            }
        });
        let sender = match self.sender {
            Some(s) => s,
            None => panic!("Sender was not established"),
        };

        let _ = sender.send(OwnedMessage::Text(String::from(format!(
            "PASS oauth:{}",
            self.creds.oauth
        ))));
        let _ = sender.send(OwnedMessage::Text(String::from(format!(
            "NICK {}",
            self.creds.user_name
        ))));
        loop {
            match rx_main.recv() {
                Ok(succ) => return Ok(succ),
                Err(e) => {
                    return Err(e);
                }
            };
        }
    }
}
