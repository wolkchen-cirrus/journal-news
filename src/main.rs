use std::{env, fs, error::Error};
use serde::Deserialize;
use toml;


#[derive(Debug, Deserialize)]
struct Paths {
    journal_dict: String,
}

#[derive(Debug, Deserialize)]
struct List {
    hours: i8,
}

#[derive(Deserialize, Debug)]
struct Config {
    paths: Paths, 
    list: List,
}

fn get_xml (s: String) -> String {
    let out = String::from(s);
        return out;
    }

fn read_config () -> Result<Config, Box<dyn Error>> {
    let config: Config = {
        let config_text = fs::read_to_string("~/.journal-news.toml")?;
        toml::from_str(&config_text)?
    };
    return Ok(config);
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {
    let conf = read_config()?;
    dbg!(conf);
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    dbg!(action);
    let test = String::from("test");
    let xml = get_xml(test);
    dbg!(xml);
    Ok(())
}

