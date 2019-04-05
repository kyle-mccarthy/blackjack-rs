// bank roll is in cents
pub struct Player {
    name: Option<String>,
}

impl Default for Player {
    fn default() -> Player {
        Player { name: None }
    }
}

impl Player {
    pub fn new() -> Player {
        Default::default()
    }

    pub fn new_with_name(name: String) -> Player {
        Player { name: Some(name) }
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name)
    }
}

pub trait NamedPlayer {
    fn get_name(&self) -> &Option<String>;
    fn set_name(&mut self, name: String);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_player_name() {
        let mut player = Player::new();

        player.set_name(String::from("Test"));

        assert!(player.get_name().eq(&Some(String::from("Test"))));
    }
}
