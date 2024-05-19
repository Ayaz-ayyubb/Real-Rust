use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton};
use serde_json::json;
use teloxide::types::{ParseMode, UpdateKind};

async fn airing(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    let message = cx.update;

    if let UpdateKind::Message(message) = &message.kind {
        if let Some(search) = message.get_text() {
            let search_str: Vec<&str> = search.splitn(2, ' ').collect();
            if search_str.len() == 1 {
                return cx.answer("Tell Anime Name :) ( /airing <anime name>)").await;
            }
            let variables = json!({"search": search_str[1]});
            let response = match reqwest::Client::new()
                .post(url)
                .json(&json!({"query": airing_query, "variables": variables}))
                .send()
                .await
            {
                Ok(res) => res.json::<serde_json::Value>().await.unwrap(),
                Err(_) => return cx.answer("Error fetching data").await,
            };
            let response = response.get("data").and_then(|d| d.get("Media"));

            if let Some(response) = response {
                let title_romaji = response["title"]["romaji"].as_str().unwrap_or("");
                let title_native = response["title"]["native"].as_str().unwrap_or("");
                let id = response["id"].as_str().unwrap_or("");
                let mut msg = format!(
                    "*Name*: *{}*(`{}`)\n*ID*: `{}`",
                    title_romaji, title_native, id
                );

                if let Some(next_airing_episode) = response.get("nextAiringEpisode") {
                    let episode = next_airing_episode["episode"].as_i64().unwrap_or(0);
                    let time_until_airing = next_airing_episode["timeUntilAiring"]
                        .as_i64()
                        .unwrap_or(0)
                        * 1000; // Assuming time is in milliseconds
                    let time = t(time_until_airing);
                    msg += &format!(
                        "\n*Episode*: `{}`\n*Airing In*: `{}`",
                        episode, time
                    );
                } else {
                    let episodes = response["episodes"].as_i64().unwrap_or(0);
                    msg += &format!("\n*Episode*:{}`\n*Status*: `N/A`", episodes);
                }

                return cx.answer(msg).parse_mode(ParseMode::Markdown).await;
            }
        }
    }

    cx.answer("Invalid command").await
}

// Function to format time (t) can be implemented here.

// Assuming these constants are defined somewhere in your code.
const url: &str = "https://graphql.anilist.co";
const airing_query: &str = "
    query ($id: Int, $search: String) { 
        Media (id: $id, type: ANIME, search: $search) { 
            id
            episodes
            title {
                romaji
                english
                native
            }
            nextAiringEpisode {
                airingAt
                timeUntilAiring
                episode
            } 
        }
    }
";

async fn anime(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    // Implementation for the anime command
}

async fn character(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    // Implementation for the character command
}

async fn manga(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    // Implementation for the manga command
}

async fn handle_commands(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    // Parse the command and call the corresponding handler
    if let Some(text) = cx.update.text() {
        let mut split = text.splitn(2, ' ');
        let command = split.next().unwrap_or("");
        let args = split.next().unwrap_or("");

        match command {
            "/airing" => airing(cx).await,
            "/anime" => anime(cx).await,
            "/character" => character(cx).await,
            "/manga" => manga(cx).await,
            _ => Ok(cx.answer("Invalid command").await?),
        }
    } else {
        Ok(cx.answer("Invalid command").await?)
    }
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(handle_commands)
        .dispatch()
        .await;
}
