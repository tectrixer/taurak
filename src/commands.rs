use crate::checks::author_has_permission;
use crate::checks::ParsedCommand;
use crate::chess_game::render_board;
use serenity::{model::channel::Message, prelude::*};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub fn get_commands() -> HashMap<
    String,
    fn(Context, Message, ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
> {
    let mut functions: HashMap<
        String,
        fn(
            Context,
            Message,
            ParsedCommand,
        ) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
    > = HashMap::<
        String,
        fn(
            Context,
            Message,
            ParsedCommand,
        ) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
    >::new();
    functions.insert("ping".into(), ping);
    functions.insert("dm_not_implemented".into(), dm_not_implemented);
    functions.insert("no command".into(), no_command);
    functions.insert("test".into(), test);
    // functions.insert("lyr".into(), lyr);
    // functions.insert("lyrics".into(), lyr);
    functions.insert("lol".into(), lol);
    functions.insert("xD".into(), xd);
    functions.insert("xd".into(), xd);
    functions.insert("XD".into(), xd);
    functions.insert("Sören".into(), soeren);
    functions.insert("Sö".into(), soeren);
    functions.insert("Söse".into(), soeren);
    functions.insert("admin".into(), admin);
    functions.insert("am_i_admin".into(), admin);
    functions.insert("Admin".into(), admin);
    functions.insert("Administrator".into(), admin);
    functions.insert("administrator".into(), admin);
    functions.insert("offline".into(), offline);
    functions.insert("online".into(), online);
    functions.insert("chess".into(), chess);
    functions.insert("def".into(), define);
    functions.insert("define".into(), define);
    // TODO: Maybe handle aliases in a better more efficient way

    return functions;
}

pub fn ping(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(ping_async(ctx, msg, args))
}

async fn ping_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn dm_not_implemented(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(dm_not_implemented_async(ctx, msg, args))
}

async fn dm_not_implemented_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Sorry, but I do not have any functions for DM's yet.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn no_command(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(no_command_async(ctx, msg, args))
}

async fn no_command_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "You need to specify a command after the prefix, otherwise I do not know what to do.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn test(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(test_async(ctx, msg, args))
}

async fn test_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hello, im online :)").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn _lyr(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(_lyr_async(ctx, msg, args))
}

async fn _lyr_async(ctx: Context, msg: Message, args: ParsedCommand) {
    use std::process::Command;

    let song = match args.args {
        Some(args) => args.join(" "),
        _ => "".into(),
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("lyr-no-prompt query {}", song))
        .output()
        .expect("Error using the lyr command");

    let hello =
        String::from_utf8(output.stdout).expect("Error when converting command output to string");
    if hello.chars().count() == 0 {
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                "You need to specify a song otherwise I do not know what to search for :(",
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    } else {
        let paragraphs: Vec<&str> = hello.split("\n\n").collect();
        for paragraph in paragraphs {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("{}", paragraph))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

pub fn lol(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(lol_async(ctx, msg, args))
}

async fn lol_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "When I was young we used to say ROFL instead of lol.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}
pub fn xd(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(xd_async(ctx, msg, args))
}

async fn xd_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "You can also use emojis, they are better for showing your emotions",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}
pub fn soeren(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(soeren_async(ctx, msg, args))
}

async fn soeren_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "Scheel\nDer einzig Wahre...")
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn admin(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(admin_async(ctx, msg, args))
}

async fn admin_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(
        msg.clone(),
        serenity::model::permissions::Permissions::ADMINISTRATOR,
        &ctx.clone().cache,
        ctx.clone(),
    )
    .await
    {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Yes, you are the admin, my Lord")
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "No, you do not possess the power of god")
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}
pub fn offline(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(offline_async(ctx, msg, args))
}

async fn offline_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(
        msg.clone(),
        serenity::model::permissions::Permissions::ADMINISTRATOR,
        &ctx.clone().cache,
        ctx.clone(),
    )
    .await
    {
        if let Err(why) = msg.channel_id.say(&ctx.http, "It's getting dark...").await {
            println!("Error sending message: {:?}", why);
        }
        ctx.clone().invisible().await;
    } else {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Sorry, only for admins :(")
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub fn online(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(online_async(ctx, msg, args))
}

async fn online_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(
        msg.clone(),
        serenity::model::permissions::Permissions::ADMINISTRATOR,
        &ctx.clone().cache,
        ctx.clone(),
    )
    .await
    {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Ahh, the sun is rising...")
            .await
        {
            println!("Error sending message: {:?}", why);
        }
        ctx.clone().online().await;
    } else {
        if let Err(why) = msg
            .channel_id
            .say(&ctx.http, "Sorry, only for admins :(")
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

// TODO: add chessrs and implement displaying a board with figures && later also adding possibility to move
pub fn chess(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(chess_async(ctx, msg, args))
}

async fn chess_async(ctx: Context, msg: Message, args: ParsedCommand) {
    if match &args.args {
        Some(v) => v.len() == 0,
        _ => true,
    } {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Ahh, you want to play chess...\nI need some more information to start a match:\nIf you want to play against a bot enter ```t chess bot```").await {
        println!("Error sending message: {:?}", why);
    }
    } else if match &args.args {
        Some(v) => v.len() > 2 && v[0] == "fen",
        _ => false,
    } {
        render_board(
            ctx,
            msg,
            &args
                .clone()
                .args
                .expect("Somehow logic does not work anymore")[1],
            &args.args.expect("There should be a third argument")[2],
        )
        .await;
    } else if match &args.args {
        Some(v) => v.len() == 2 && v[0] == "fen",
        _ => false,
    } {
        render_board(
            ctx,
            msg,
            &args
                .clone()
                .args
                .expect("Somehow logic does not work anymore")[1],
            "",
        )
        .await;
    } else {
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, I can only display FEN positions yet:\n``` t chess fen <position>```",
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub fn define(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(define_async(ctx, msg, args))
}

async fn define_async(ctx: Context, msg: Message, args: ParsedCommand) {
    let search_word = match args.args {
        Some(x) => {
            if x.len() > 0 {
                x.join(" ")
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "I need a definition to search for:\n``` t define <word>```",
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
        }
        _ => {
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    "I need a definition to search for:\n``` t define <word>```",
                )
                .await
            {
                println!("Error sending message: {:?}", why);
            }
            return;
        }
    };
    let response = match get_definition(&search_word).await {
        Ok(definition) => definition,
        Err(why) => {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("There has been an error:\n```{}```", why))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
            return;
        }
    };
    if let Err(_) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}", response.word));
                e.field("Description:".to_string(), response.description, false);
                e.field("Example:".to_string(), response.example, false);
                e.field("Author:".to_string(), response.author, false);

                return e;
            });

            return m;
        })
        .await
    {
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, there was an error printing the definition, it might have been to long :(",
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}

// TODO: add help command (maybe automatic implementation)
use std::fmt;

#[derive(Debug, Clone)]
struct UrbanError;
impl fmt::Display for UrbanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error when getting definition, probably it does not exist yet :("
        )
    }
}
async fn get_definition(search_word: &str) -> Result<Definition, UrbanError> {
    use serde_json::Value;
    let res = reqwest::get(&format!(
        "https://api.urbandictionary.com/v0/define?term={}",
        search_word
    ))
    .await
    .expect("Error connecting");
    let body = res.text().await.expect("error getting body");
    let value: Value = serde_json::from_str(&body).expect("");
    Ok(Definition {
        word: match value["list"][0]["word"].as_str() {
            Some(x) => x,
            _ => return Err(UrbanError),
        }
        .to_string()
        .replace("[", "")
        .replace("]", ""),
        description: match value["list"][0]["definition"].as_str() {
            Some(x) => x,
            _ => return Err(UrbanError),
        }
        .to_string()
        .replace("[", "")
        .replace("]", ""),
        example: match value["list"][0]["example"].as_str() {
            Some(x) => x,
            _ => return Err(UrbanError),
        }
        .to_string()
        .replace("[", "")
        .replace("]", ""),
        author: match value["list"][0]["author"].as_str() {
            Some(x) => x,
            _ => return Err(UrbanError),
        }
        .to_string()
        .replace("[", "")
        .replace("]", ""),
    })
}

#[derive(Debug)]
struct Definition {
    word: String,
    description: String,
    example: String,
    author: String,
}
