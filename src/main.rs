fn main() {
    println!("Hello, world!");

    let outlet = Outlet::new(String::from("Outlet in the hall"));

    println!("Outlet: '{}'", outlet.get_description());
    println!("\tPower consumption: {}", outlet.get_power_consumption());
    println!("\tIs turned on: {}", outlet.turned_on);

    let thermometer = Thermometer::new();

    println!("Thermometer:");
    println!("\tTemperature: {}", thermometer.get_temperature());
}

struct Outlet {
    description: String,
    power_consumption: u32,
    turned_on: bool,
}

impl Outlet {
    fn new(description: String) -> Outlet {
        Outlet {
            description,
            power_consumption: 0,
            turned_on: false,
        }
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_power_consumption(&self) -> u32 {
        self.power_consumption
    }
}

struct Thermometer {
    temperature: i32,
}

impl Thermometer {
    fn new() -> Thermometer {
        Thermometer { temperature: 0 }
    }

    fn get_temperature(&self) -> i32 {
        self.temperature
    }
}
