use std::{env, fs, io};
use serde::Deserialize;
use toml;


#[derive(Debug, Deserialize)]
struct Config {
    num: i8,
    foo: String, 
}

fn get_xml (s: String) -> String {
    let out = String::from(s);
    return out;
}

fn read_config() -> io::Result<Config> {
    let content = fs::read_to_string("~/.journal-news")?;
    Ok(toml::from_str(&content)?)
}

#[tokio::main]
async fn main () {
    let conf = read_config().expect("Could not read settings file");
    dbg!(conf);
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    dbg!(action);
    let test = String::from("test");
    let xml = get_xml(test);
    dbg!(xml);
}

