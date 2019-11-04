use std::error::Error;
use std::fs;

use crate::character::Character;
use crate::ui::MainApp;

mod ui;
mod character;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub struct Config {
    pub character_filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments")
        }

        let character_filename = args[1].clone();

        Ok(Config { character_filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let dandelion_json = fs::read_to_string(config.character_filename)?;

    let dandelion: Character = serde_json::from_str(&dandelion_json)?;

    let main_app = MainApp::new ( &dandelion )?;

    main_app.draw_app()?;

    Ok(())
}