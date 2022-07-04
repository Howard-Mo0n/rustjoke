use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Flags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Joke {
    error: bool,
    category: String,
    type_: Option<String>,
    setup: Option<String>,
    delivery: Option<String>,
    joke: Option<String>,
    flags: Flags,
    safe: bool,
    id: i32,
    lang: String,
}

impl Joke {
    async fn get(safe_mode: bool) -> Result<Self, ExitFailure> {
        let url: String = format!(
            "https://v2.jokeapi.dev/joke/Any{}",
            if safe_mode { "?safe-mode" } else { "" }
        );

        let url: Url = Url::parse(&*url)?;
        let res: Joke = reqwest::get(url).await?.json::<Joke>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let mut safe_mode: bool = false;

    let args: Vec<String> = env::args().collect();
    if args.contains(&"--safe".to_string()) {
        safe_mode = true;
    }

    let res: Joke = Joke::get(safe_mode).await?;

    match res.setup {
        Some(_) => println!("{} \n {}", res.setup.unwrap(), res.delivery.unwrap()),
        None => println!("{}", res.joke.unwrap()),
    }

    Ok(())
}
