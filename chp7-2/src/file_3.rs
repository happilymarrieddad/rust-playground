mod travel_history {
    pub enum Conveyance {
        Car,
        Train,
        Air,
    }
}

pub fn allowance() {
    let travel_1 = travel_history::Conveyance::Car;
}
