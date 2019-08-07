extern crate serenity;

use std::{env};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

fn uwu_replacer(original: String) -> String {
    let uwu_letters = "rl";
    let happy_dance = "uwu YOU SAID UWU ♪┏(・o･)┛♪┗ ( ･o･) ┓♪";
    let mut better_string = String::new();
    for c in original.chars() {
        if uwu_letters.contains(c) {
            better_string += "w"
        } else {
            better_string += &c.to_string()
        }
    }
    let best_string= better_string
        .replace("th", "d")
        .replace("ou", "ew")
        .replace("uwu", happy_dance)
        .replace("uWu", happy_dance);
    String::from(best_string)
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        println!("new message incoming!");
        let mut uwud = String::new();
        if msg.content.starts_with("!uwu") ||  msg.content.starts_with("@uwu") {
            let mut reply = String::new();
            let ref_of_message = &msg.content;
            match ref_of_message.get(4..) {
                Some(my_str) => reply += my_str,
                None => reply += format!("{} uwud!", msg.author.name).as_str()
            }
            uwud += format!("{} ᴜwᴜ", uwu_replacer(reply)).as_str();

            // The create message builder allows you to easily create embeds and messages
            // using a builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, three fields, and a footer.
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(uwud);
                m
            });

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}