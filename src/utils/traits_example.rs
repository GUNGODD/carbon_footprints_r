struct Sedon;
impl Sedon {
    fn drive(&self) {
        println!("Sedon is driving ");
    }
}

fn road_trip(vehicle: &Sedon) {
    vehicle.drive();
}

pub fn traits_main() {
    let car = Sedon;

    car.drive();
    road_trip(&car);
}
