extern crate serenity;

use std::{env};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

fn uwu_replacer(original: &String) -> String {
    let uwu_letters = "rl";
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
        .replace("qu", "qw")
        .replace("uwu", "ᴜwᴜ")
        .replace("uWu", "ᴜwᴜ");
    String::from(best_string)
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // println!("{}", msg.content);
        let mut uwud = String::new();
        if !msg.author.bot 
                && msg.content.contains("uwu") 
                ||  msg.content.contains(" uWu ")             
            {

            uwud += format!("<@{}> uwud! ♪┏(・o･)┛♪┗ ( ･o･) ┓♪ ᴜwᴜ \n {}", msg.author.id, uwu_replacer(&msg.content)).as_str();

            // The create message builder allows you to easily create embeds and messages
            // using a builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, three fields, and a footer.
            let res = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(uwud);
                m
            });
            let _ = msg.delete(&ctx);

            if let Err(why) = res {
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