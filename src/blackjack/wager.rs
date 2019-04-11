pub struct Wager {
    wager: u32,
}

impl Wager {
    pub fn new() -> Wager {
        Wager {
            wager: 0,
        }
    }

    pub fn set_wager(&mut self, wager: u32) {
        self.wager = wager;
    }

    pub fn get_wager(&self) -> u32 {
        self.wager
    }

    pub fn reset_wager(&mut self) {
        self.wager = 0;
    }

    pub fn add_wager(&mut self, wager: u32) {
        self.wager += wager;
    }
}

pub trait WithWager {
    fn get_mut_wager(&mut self) -> &mut Wager;
    fn get_wager(&self) -> &Wager;

    fn set_wagered_value(&mut self, wager: u32) {
        self.get_mut_wager().set_wager(wager)
    }

    fn get_wagered_value(&self) -> u32 {
        self.get_wager().get_wager()
    }

    fn reset_wagered_value(&mut self) {
        self.get_mut_wager().reset_wager()
    }

    fn add_wager_to_wagered_value(&mut self, wager: u32) {
        self.get_mut_wager().add_wager(wager);
    }
}
