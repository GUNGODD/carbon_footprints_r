struct RacingCars {
    modal: String,
    year: u32,
}

impl RacingCars {
    fn mercedes(&self) {
        println!("Mercedes is running........");
    }
}

pub fn traits_main() {
    let car = RacingCars {
        modal: "Mercedes".to_string(),
        year: 2022,
    };
    car.mercedes();
}
