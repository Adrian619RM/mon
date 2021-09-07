use std::env;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let request_text = get_pokemon_info(&args[1]).await?;
    let pokemon = parse_pokemon_info(&request_text).await?;
    let colorscript = get_pokemon_colorscript(&pokemon.name).await?;

    print_pokemon(&pokemon, &colorscript).await;

    Ok(())
}

struct Pokemon {
    id: u16,
    name: String,
}

async fn get_pokemon_info(identifier: &String) -> reqwest::Result<String> {
    let res = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", identifier)).await?;
    let text = res.text().await?;

    Ok(text)
}

// TODO request Pokemon:Pokemon instead of Pokemon:Species which includes needed data
async fn parse_pokemon_info(info: &String) -> serde_json::Result<Pokemon> {
    let v: Value = serde_json::from_str(&info)?;

    let pokemon = Pokemon {
        id: {
            let x = v["id"].to_string();
            x.parse::<u16>().unwrap()
        },
        name:  v["name"].to_string().to_lowercase().replace("\"", ""),
    };

    Ok(pokemon)
}

async fn get_pokemon_colorscript(name: &String) -> std::io::Result<Vec<String>> {
    let mut file = File::open(format!("colorscripts/{}.txt", name))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents = contents.lines().map(String::from).collect::<Vec<_>>();

    Ok(contents)
}

// See previous TODO comment
async fn print_pokemon(pokemon: &Pokemon, colorscript: &Vec<String>) {
    todo!();
}
