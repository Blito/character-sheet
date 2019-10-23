use std::io;
use crate::character::Character;
use crate::ui::MainApp;
use std::collections::HashMap;

mod ui;
mod character;

fn main() -> Result<(), io::Error> {

    let dandelion = Character::new (
        String::from("Dandelion"),
        String::from("Rock Gnome"),
        String::from("Wizard"),
        3,
        17,
        17,
        HashMap::new(),
        13,
        3,
        0,
        25
    );

    let main_app = MainApp::new ( &dandelion )?;

    main_app.draw_app()
}