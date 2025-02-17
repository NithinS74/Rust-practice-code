enum WEATHER {
    Sunny,
    Rainy,
    Cloudy,
}

fn main() {
    let x: WEATHER = WEATHER::Sunny;
    weather_forecast(x);
}

fn weather_forecast(x: WEATHER) {
    match x {
        WEATHER::Sunny => println!("Its a sunny day"),
        WEATHER::Rainy => println!("Its a rainy day"),
        WEATHER::Cloudy => println!("Its a cloud day"),
    }
}
