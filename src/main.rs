use std::{env, fs, error::Error, path::Path, collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use home;

#[derive(Serialize, Deserialize)]
struct Vars {
    config: HashMap<String, Value>,
    urls: HashMap<String, Value>,
}

fn get_xml (s: String) -> String {
    let out = String::from(s);
        return out;
    }

fn read_config () -> Result<Vars, Box<dyn Error>> {
    let hd = home::home_dir().unwrap();
    let settings_path =  Path::new(&hd).join(".journal-news.json");
    let config: Vars = {
        let config_text = fs::read_to_string(&settings_path)?;
        serde_json::from_str(&config_text)?
    };
    return Ok(config);
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {
    let inputs = read_config()?;
    let test1: &Value = inputs.config.get("hours").unwrap();
    println!("{:?}", test1);
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let test = String::from("test");
    let xml = get_xml(test);
    Ok(())
}

