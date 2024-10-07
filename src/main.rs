use std::io;
use colored::*;
use serde::Deserialize;
use reqwest::blocking::get;

// Struct to store weather information obtained from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherData {
    weather: Vec<WeatherDetails>, // Contains description of the weather
    main: WeatherMain,            // Holds core weather metrics
    wind: WindInfo,               // Contains wind-related data
    name: String,                 // Holds the location name
}

// Struct representing weather description details
#[derive(Deserialize, Debug)]
struct WeatherDetails {
    description: String, // Describes the weather condition
}

// Struct representing main weather parameters
#[derive(Deserialize, Debug)]
struct WeatherMain {
    temp: f64,     // Temperature in Celsius
    humidity: f64, // Humidity percentage
    pressure: f64, // Pressure in hPa
}

// Struct representing wind information
#[derive(Deserialize, Debug)]
struct WindInfo {
    speed: f64, // Speed of the wind in m/s
}

// Core struct responsible for retrieving and displaying weather data
struct WeatherApp {
    api_token: String, // API token for OpenWeatherMap access
}

impl WeatherApp {
    // Constructs a new instance of WeatherApp
    fn initialize(api_token: &str) -> Self {
        WeatherApp {
            api_token: api_token.to_owned(),
        }
    }

    // Retrieves weather data from the API using the specified city and country code
    fn obtain_weather(&self, city: &str, country: &str) -> Result<WeatherData, reqwest::Error> {
        let api_endpoint = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city, country, self.api_token
        );

        let api_response = get(&api_endpoint)?;
        let weather_info = api_response.json::<WeatherData>()?;
        Ok(weather_info)
    }

    // Displays the weather details in a formatted way
    fn render_weather_info(&self, weather_info: &WeatherData) {
        let weather_desc = &weather_info.weather[0].description;
        let temp = weather_info.main.temp;
        let humidity = weather_info.main.humidity;
        let pressure = weather_info.main.pressure;
        let wind_velocity = weather_info.wind.speed;

        let formatted_details = format!(
            "Weather Update for {}: {} {}
            > Temperature: {:.1}°C
            > Humidity: {:.1}%
            > Pressure: {:.1} hPa
            > Wind Speed: {:.1} m/s",
            weather_info.name,
            weather_desc,
            Self::emoji_for_temperature(temp),
            temp,
            humidity,
            pressure,
            wind_velocity
        );

        let colored_output = Self::colorize_weather_output(weather_desc, &formatted_details);
        println!("{}", colored_output);
    }

    // Determines an emoji representation based on the temperature
    fn emoji_for_temperature(temp: f64) -> &'static str {
        match temp {
            _ if temp < 0.0 => "❄️",
            _ if temp < 10.0 => "☁️",
            _ if temp < 20.0 => "⛅",
            _ if temp < 30.0 => "🌤️",
            _ => "🔥",
        }
    }

    // Applies color effects to the weather report based on the description
    fn colorize_weather_output(description: &str, weather_text: &str) -> ColoredString {
        match description {
            "clear sky" => weather_text.bright_yellow(),
            "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
            "overcast clouds" | "mist" | "haze" | "smoke" | "dust" | "fog" => weather_text.dimmed(),
            "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
            _ => weather_text.normal(),
        }
    }
}

// Handles interactions with the user in the terminal
struct UserInteraction;

impl UserInteraction {
    // Prompts the user to enter the city and country code
    fn acquire_user_input() -> (String, String) {
        println!("{}", "Enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Unable to read city name");
        let city = city.trim().to_string();

        println!("{}", "Enter the country code (e.g., US for United States):".bright_green());
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Unable to read country code");
        let country = country.trim().to_string();

        (city, country)
    }

    // Main execution loop to fetch weather data and handle user prompts
    fn execute_app(weather_app: &WeatherApp) {
        println!("{}", "Welcome to Weather App!".bright_yellow());

        loop {
            let (city, country) = Self::acquire_user_input();

            match weather_app.obtain_weather(&city, &country) {
                Ok(weather_info) => weather_app.render_weather_info(&weather_info),
                Err(e) => eprintln!("Error retrieving weather information: {}", e),
            }

            println!("{}", "Would you like to check the weather for another location? (yes/no):".bright_green());
            let mut user_choice = String::new();
            io::stdin().read_line(&mut user_choice).expect("Unable to read user input");
            if user_choice.trim().to_lowercase() != "yes" {
                println!("Thank you for using Weather App!");
                break;
            }
        }
    }
}

fn main() {
    let api_token = ""; // <-- API KEY
    let weather_app = WeatherApp::initialize(api_token);

    UserInteraction::execute_app(&weather_app);
}
