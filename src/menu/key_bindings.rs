use serde::{Deserialize, Serialize};
use minifb::Key;
use minifb::Window;
use std::str::FromStr;

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

    pub fn change_key(&mut self, action: &str, key: &str,) -> Result<(), String> {
        if !self.key_valid(key) {
            return Err(format!("Invalid key: {}", key));
        }
        let binding = self.key_bindings
                                                        .iter_mut()
                                                        .find(|(a, _)| a == action);
        match binding {
            Some((_, current_key)) => {
                *current_key = key.to_string();
                Ok(())
            }
            None => Err(format!("Action not found: {}", action)),
        }
        
    }

    pub fn key_valid(&self, key: &str) -> bool {

        return !key.is_empty() 
            && !self.key_bindings.iter().any(|(_, k)| k == key);
    }

    //wird gedrückt gehalten
    pub fn is_action_pressed(&self, window: &Window, action: &str) -> bool {
        let key = self.key_bindings.iter()
            .find(|(a, _)| a == action)
            .map(|(_, key)| key);

        let Some(key) = key else {
            return false;
        };

        window.get_keys_pressed(minifb::KeyRepeat::No)
            .iter()
            .any(|pressed| format!("{:?}", pressed) == *key)
        
    }

    //wird einmal runtergedrückt
    pub fn is_action_down(&self, window: &Window, action: &str) -> bool {
        let key = self.key_bindings.iter()
            .find(|(a, _)| a == action)
            .map(|(_, key)| key);

        let Some(key) = key else {
            return false;
        };
        
        window.get_keys().iter()
            .any(|down| format!("{:?}", down) == *key)
    }


    /*
    pub fn get_key(&self, action: &str) -> Option<Key> {
        self.key_bindings
            .iter()
            .find(|(a, _)| a == action)
            .unwrap().1;
            //unschön 
    }
    */

    //pub fn get_key_to_action(action: &str) -> Result<&str, Box<dyn std::error::Error>>
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


