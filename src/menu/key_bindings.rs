use serde::{Deserialize, Serialize};
use minifb::Key;

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

    //wird einmal gedrückt
    pub fn is_action_pressed(&self, keys_pressed: &[minifb::Key], action: &str) -> bool {
        let key = self.get_key_to_action(action);

        let Some(key) = key else {
            return false;
        };

        keys_pressed
            .iter()
            .any(|pressed| format!("{:?}", pressed) == *key)
        
    }

    //wird gedrückt gehalten
    pub fn is_action_down(&self, keys_down: &[minifb::Key], action: &str) -> bool {
        let key = self.get_key_to_action(action);

        let Some(key) = key else {
            return false;
        };
        
        keys_down
            .iter()
            .any(|down| format!("{:?}", down) == *key)
    }

    pub fn get_key_to_action(&self, action: &str) -> Option<&String>{
        self.key_bindings.iter()
            .find(|(a, _)| a == action)
            .map(|(_, key)| key)
    }

}




