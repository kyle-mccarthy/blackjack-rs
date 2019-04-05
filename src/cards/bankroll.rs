pub struct Bankroll {
    balance: u32,
}

impl Default for Bankroll {
    fn default() -> Bankroll {
        Bankroll { balance: 0 }
    }
}

impl Bankroll {
    pub fn new() -> Bankroll {
        Default::default()
    }

    pub fn set_bankroll(&mut self, bankroll: u32) {
        self.balance = bankroll;
    }

    pub fn get_bankroll(&self) -> u32 {
        self.balance
    }

    pub fn sub_funds(&mut self, value: u32) -> Result<u32, &str> {
        match self.balance.checked_sub(value) {
            Some(next_value) => {
                self.balance = next_value;
                Ok(next_value)
            }
            None => Err("Insufficient funds"),
        }
    }

    pub fn add_funds(&mut self, value: u32) -> Result<u32, &str> {
        match self.balance.checked_add(value) {
            Some(next_value) => {
                self.balance = next_value;
                Ok(next_value)
            }
            None => Err("Overflow"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_player_bankroll() {
        let mut br = Bankroll::new();

        br.set_bankroll(100);

        assert_eq!(br.get_bankroll(), 100);
    }

    #[test]
    fn should_sub_player_bankroll() {
        let mut br = Bankroll::new();

        br.set_bankroll(100);

        let curr_bankroll = br.sub_funds(10);

        assert!(curr_bankroll.is_ok());
        assert_eq!(curr_bankroll.unwrap_or_default(), 90);
        assert_eq!(curr_bankroll.unwrap_or_default(), br.get_bankroll());
    }

    #[test]
    fn should_add_player_bankroll() {
        let mut br = Bankroll::new();

        br.set_bankroll(100);

        let curr_bankroll = br.add_funds(10);

        assert!(curr_bankroll.is_ok());
        assert_eq!(curr_bankroll.unwrap_or_default(), 110);
        assert_eq!(curr_bankroll.unwrap_or_default(), br.get_bankroll());
    }

    #[test]
    fn should_not_sub_insufficent_funds() {
        let mut br = Bankroll::new();

        br.set_bankroll(100);

        let result = br.sub_funds(200);

        assert!(result.is_err());
        assert_eq!(br.get_bankroll(), 100);
    }
}
