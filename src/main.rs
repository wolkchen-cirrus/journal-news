use std::{env, fs, error::Error, path::Path};
use serde_json::Value;
use home;


fn get_xml (s: String) -> String {
    let out = String::from(s);
        return out;
    }

fn read_config () -> Result<Value, Box<dyn Error>> {
    let config = {
        let hd = home::home_dir().unwrap();
        let settings_path =  Path::new(&hd).join(".journal-news.json");
        let config_text = fs::read_to_string(&settings_path)?;
        serde_json::from_str::<Value>(&config_text)?
    };
    return Ok(config);
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {
    let conf = read_config()?;
    println!("{:?}", serde_json::to_string_pretty(&conf));
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let test = String::from("test");
    let xml = get_xml(test);
    Ok(())
}

