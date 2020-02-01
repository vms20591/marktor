use clap::{Arg, App, SubCommand};
use serde::{Serialize, Deserialize};
use serde_json::{Result as SerdeResult};

#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {
    name: String,
    url: String
}

impl Bookmark {
    fn new(name: String, url: String) -> Bookmark {
        Bookmark {name, url} 
    }

    fn to_json(&self) -> SerdeResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

fn add_bookmark(name: &str, url: &str) -> SerdeResult<()> {
    let bookmark = Bookmark::new(String::from(name), String::from(url));

    let json = bookmark.to_json()?;

    println!("Bookmark: {}", json);

    Ok(())
}

fn main() -> SerdeResult<()> {
    let matches = App::new("marktor")
        .version("1.0")
        .author("Meenakshi Sundaram V <vms20591@riseup.net>")
        .about("Boomkark for tor hidden services")
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

    if let Some(sub_match) = matches.subcommand_matches("add") {
        let name = sub_match.value_of("name").unwrap();
        let url = sub_match.value_of("url").unwrap();

        add_bookmark(name, url)?;
    }

    Ok(())
}
