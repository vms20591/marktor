use std::path::Path;
use std::fs::File;
use std::io::Result;
use clap::{Arg, App, SubCommand};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Bookmark {
    name: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct BookmarkList {
    bookmarks: Vec<Bookmark>
}

struct Config {
    location: String
}

impl Bookmark {
    fn new(name: String, url: String) -> Bookmark {
        Bookmark {name, url} 
    }
}

impl Config {
    fn new(location: String) -> Config {
        Config {location} 
    }
}

impl BookmarkList {
    fn new() -> BookmarkList {
        BookmarkList {
            bookmarks: Vec::new() 
        } 
    }

    fn load(config: &Config) -> Result<BookmarkList> {
        if Path::new(&config.location).exists() {
            Ok(serde_json::from_reader(File::open(&config.location)?)?)
        } else {
            Ok(BookmarkList::new())
        }
    } 

    fn add(&mut self, bookmark: Bookmark) {
        self.bookmarks.push(bookmark);
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn save(&self, config: &Config) -> Result<()> {
        let json = self.to_json()?;

        println!("Saving bookmarks...");

        std::fs::write(&config.location, json)?;

        Ok(()) 
    }

    fn update(&mut self, name: &str, url: &str) {
        let mut found = false;

        for bookmark in self.bookmarks.iter_mut() {
            if bookmark.name == name {
                found = true;
                bookmark.url = String::from(url); 
            }
        }

        if !found {
            let bookmark = Bookmark::new(String::from(name), String::from(url));

            self.bookmarks.push(bookmark);
        }
    }

    fn delete(&mut self, name: &str) {
        let mut new_bookmarks: Vec<Bookmark> = Vec::new();

        for bookmark in self.bookmarks.iter() {
            if bookmark.name != name {
                new_bookmarks.push(Bookmark::new(bookmark.name.clone(), bookmark.url.clone()));
            }
        }

        self.bookmarks = new_bookmarks;
    }

    fn list(&self) {
        println!("Name\t\t\t\t\t\t\tURL");
        println!("====\t\t\t\t\t\t\t===");

        for bookmark in self.bookmarks.iter() {
            println!("{}\t\t\t\t\t\t{}", bookmark.name, bookmark.url);
        }
    }

    fn get(&self, name: &str) {
        println!("Name\t\t\t\t\t\t\tURL");
        println!("====\t\t\t\t\t\t\t===");

        for bookmark in self.bookmarks.iter() {
            if bookmark.name.to_ascii_lowercase().contains(&name.to_ascii_lowercase()) {
                println!("{}\t\t\t\t\t\t{}", bookmark.name, bookmark.url);
            } 
        }
    }
}

fn add_bookmark(config: &Config, bookmark_list: &mut BookmarkList, name: &str, url: &str) -> Result<()> {
    let bookmark = Bookmark::new(String::from(name), String::from(url));

    bookmark_list.add(bookmark);
    bookmark_list.save(&config)?;

    Ok(())
}

fn update_bookmark(config: &Config, bookmark_list: &mut BookmarkList, name: &str, url: &str) -> Result<()> {
    bookmark_list.update(&name, &url);
    bookmark_list.save(&config)?;
 
    Ok(())
}

fn delete_bookmark(config: &Config, bookmark_list: &mut BookmarkList, name: &str) -> Result<()> {
    bookmark_list.delete(&name);
    bookmark_list.save(&config)?;
 
    Ok(())
}

fn list_bookmark(bookmark_list: &BookmarkList) {
    bookmark_list.list();
}

fn get_bookmark(bookmark_list: &BookmarkList, name: &str) {
    bookmark_list.get(&name);
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
        .subcommand(SubCommand::with_name("update")
            .about("Update an existing bookmark")
            .arg(Arg::with_name("name")
                 .required(true)
                 .index(1)
                 .help("Name of the bookmark"))
            .arg(Arg::with_name("url")
                 .required(true)
                 .index(2)
                 .help("Hidden service url")))
        .subcommand(SubCommand::with_name("delete")
            .about("Delete an existing bookmark")
            .arg(Arg::with_name("name")
                 .required(true)
                 .index(1)
                 .help("Name of the bookmark")))
        .subcommand(SubCommand::with_name("list")
            .about("List all bookmarks"))
        .subcommand(SubCommand::with_name("get")
            .about("Get an existing bookmark")
            .arg(Arg::with_name("name")
                 .required(true)
                 .index(1)
                 .help("Name of the bookmark")))
        .get_matches();

    let location = matches.value_of("location").unwrap_or("marktor.json");

    let config = Config::new(String::from(location));

    let mut bookmark_list = BookmarkList::load(&config)?;

    if let Some(sub_match) = matches.subcommand_matches("add") {
        let name = sub_match.value_of("name").unwrap();
        let url = sub_match.value_of("url").unwrap();

        add_bookmark(&config, &mut bookmark_list, name, url)?;
    }

    if let Some(sub_match) = matches.subcommand_matches("update") {
        let name = sub_match.value_of("name").unwrap();
        let url = sub_match.value_of("url").unwrap();

        update_bookmark(&config, &mut bookmark_list, name, url)?;
    }

    if let Some(sub_match) = matches.subcommand_matches("delete") {
        let name = sub_match.value_of("name").unwrap();

        delete_bookmark(&config, &mut bookmark_list, name)?;
    }

    if let Some(_) = matches.subcommand_matches("list") {
        list_bookmark(&bookmark_list);
    }

    if let Some(sub_match) = matches.subcommand_matches("get") {
        let name = sub_match.value_of("name").unwrap();

        get_bookmark(&bookmark_list, name);
    }

    Ok(())
}
