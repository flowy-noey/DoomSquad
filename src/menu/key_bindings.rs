use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
//use std::fs;
//use std::path::Path;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyBindingList {
    pub key_bindings: Vec<(String, String)>,
}

impl KeyBindingList {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&content)?)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::default(),
        )?;

        std::fs::write(path, content)?;
        Ok(())
    }
}

/*
pub struct KeyBindings {
    forwards: String,
    backwards: String,
    sprint: String,
    crouch: String,
    interact: String,
    move_crosshair_down: String,
    turn_right: String,
    fire_bullet: String,
    godmode: String,
    main_menu: String,
    left: String,
    right: String,
    jump: String,
    move_crosshair_up: String,
    turn_left: String,
    fine_aiming: String,
    fire_arrow: String,
    lock_enemies: String,
}

*/


