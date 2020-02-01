use std::io::Result;
use clap::{Arg, App, SubCommand};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {
    name: String,
    url: String
}

struct Config {
    location: String
}

impl Bookmark {
    fn new(name: String, url: String) -> Bookmark {
        Bookmark {name, url} 
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn save(&self, config: &Config) -> Result<()> {
        let json = self.to_json()?;

        println!("Saving bookmark: {}", json);

        std::fs::write(&config.location, json)?;

        Ok(()) 
    }
}

impl Config {
    fn new(location: String) -> Config {
        Config {location} 
    }
}

fn add_bookmark(config: &Config, name: &str, url: &str) -> Result<()> {
    let bookmark = Bookmark::new(String::from(name), String::from(url));

    bookmark.save(&config)?;

    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("marktor")
        .version("1.0")
        .author("Meenakshi Sundaram V <vms20591@riseup.net>")
        .about("Boomkark for tor hidden services")
        .arg(Arg::with_name("location")
            .index(1)
            .help("Location of bookmark file. default: ./marktor.json"))
        .subcommand(SubCommand::with_name("add")
            .about("Add new bookmark")
            .arg(Arg::with_name("name")
                 .required(true)
                 .index(1)
                 .help("Name for the bookmark"))
            .arg(Arg::with_name("url")
                 .required(true)
            .index(2)
                 .help("Hidden service url")))
        .get_matches();

    let location = matches.value_of("location").unwrap_or("marktor.json");

    let config = Config::new(String::from(location));

    if let Some(sub_match) = matches.subcommand_matches("add") {
        let name = sub_match.value_of("name").unwrap();
        let url = sub_match.value_of("url").unwrap();

        add_bookmark(&config, name, url)?;
    }

    Ok(())
}
