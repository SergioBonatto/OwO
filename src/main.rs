use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::{
    // macros::{command, group, help},
    macros::{command, group},
    Args, CommandResult
};
use reqwest::Url;

use serenity::{
    model::{channel::Message},
};
use dotenv::dotenv;

// dotenv::dotenv().ok();
// dotenv().ok();

const GITHUB_REPO_URL: &str = "https://raw.githubusercontent.com/HigherOrderCO/Wikind/master";

#[group]
#[commands(view)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

// #[help("my_help")]
// #[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! Salve favela! 您好! 여보세요! Здравствуйте! Здра́во!"]
// #[command_not_found_text = "Could not find: `{}`."]
// #[max_levenshtein_distance(3)]
// #[strikethrough_commands_tip_in_dm(false)]
// #[strikethrough_commands_tip_in_guild(true)]
// #[embed_success_colour("#00FF00")]
// #[embed_error_colour("#FF0000")]
// async fn my_help(
//     context: &Context,
//     msg: &Message,
//     args: Args,
//     options: &HelpOptions,
//     groups: &[&CommandGroup],
//     owners: HashSet<UserId>,
// ) -> serenity::Result<()> {
//     help_commands::with_embeds(context, msg, args, options, groups, owners).await;
//     Ok(())
// }


#[command]
#[aliases("view")]
async fn view(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let filename = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Please provide a filename.").await?;
            return Ok(());
        }
    };

    let url = match Url::parse(&format!("{}/{}.kind2", GITHUB_REPO_URL, filename)) {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Invalid filename.").await?;
            return Ok(());
        }
    };

    let content = match reqwest::get(url).await {
        Ok(response) => response.text().await.unwrap(),
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Failed to fetch file.").await?;
            return Ok(());
        }
    };

    let response = format!("```{}\n{}```", filename, content);
    msg.channel_id.say(&ctx.http, response).await?;
    Ok(())
}
    
#[tokio::main]
async fn main() {
    let framework = serenity::framework::standard::StandardFramework::new()
        // .help(&MY_HELP)
        //
        .configure(|c| c.prefix("~"));
    dotenv().ok();
    // let token = std::env::var_os("MINHA_CHAVE").unwrap();
//     let token = match std::env::var_os("MINHA_CHAVE") {
//     Some(val) => val,
//     None => panic!("MINHA_CHAVE não definida"),
// };
//
// let mut client = serenity::Client::builder(token)
//         .framework(framework)
//         .await
//         .expect("Error creating client");
//
let token = std::env::var_os("MINHA_CHAVE")
    .expect("Failed to get MINHA_CHAVE from environment")
    .to_string_lossy()
    .to_string();

let mut client = serenity::Client::builder(&token)
    .framework(framework)
    .await
    .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

