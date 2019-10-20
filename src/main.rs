use std::io;
use crate::character::Character;
use crate::ui::MainApp;

mod ui;
mod character;

fn main() -> Result<(), io::Error> {

    let dandelion = Character { name: String::from("Dandelion") };

    let main_app = MainApp { character: &dandelion };

    main_app.draw_app()
}