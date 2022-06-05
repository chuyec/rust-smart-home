fn main() {
    println!("Hello, world!");

    let outlet = outlet_create(String::from("Outlet in the hall"));

    println!("Outlet: '{}'", outlet.get_description());
    println!("\tPower consumption: {}", outlet.get_power_consumption());
    println!("\tIs turned on: {}", outlet.turned_on);

    let thermometer = thermometer_create();

    println!("Thermometer:");
    println!("\tTemperature: {}", thermometer.get_temperature());
}

struct Outlet {
    description: String,
    power_consumption: u32,
    turned_on: bool,
}

impl Outlet {
    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_power_consumption(&self) -> u32 {
        self.power_consumption
    }
}

fn outlet_create(description: String) -> Outlet {
    Outlet {
        description,
        power_consumption: 0,
        turned_on: false,
    }
}

struct Thermometer {
    temperature: i32,
}

impl Thermometer {
    fn get_temperature(&self) -> i32 {
        self.temperature
    }
}

fn thermometer_create() -> Thermometer {
    Thermometer { temperature: 0 }
}
