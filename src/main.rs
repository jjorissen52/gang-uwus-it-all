extern crate discord;
//extern crate regex;

use discord::Discord;
use discord::model::Event;
//use regex::Regex;
use std::env;

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

fn main() {
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        &env::var("DISCORD_TOKEN").expect("Expected token"),
    ).expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);

                if message.content.starts_with("!uwu") ||  message.content.starts_with("@uwu") {
                    let mut reply = String::new();
                    let ref_of_message = &message.content;
                    match ref_of_message.get(5..) {
                        Some(my_str) => reply += my_str,
                        None => reply += format!("{} uwud!", message.author.name).as_str()
                    }
                    let uwud = format!("{} ᴜwᴜ", uwu_replacer(reply));
                    let _ = discord.send_message(message.channel_id, uwud.as_ref(), "", false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}