use colored::*;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use text_io::read;

#[derive(Deserialize, Debug)]
struct GeocodeResponse {
    display_name: Option<String>,
    lat: String,
    lon: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: Option<CurrentWeather>,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: Option<f64>,
    wind_speed_10m: Option<f64>,
    precipitation_probability: Option<f64>,
}

async fn fetch_geocode(city: &str) -> Result<Vec<GeocodeResponse>, Box<dyn Error>> {
    let client = Client::builder().user_agent("MyGeocodingApp/1.0").build()?;

    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&addressdetails=1",
        city
    );

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<GeocodeResponse>>() // Обработка ответа как вектор GeocodeResponse
        .await?;

    Ok(response)
}

async fn fetch_weather_data(lat: &str, lon: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let client = Client::new();
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,precipitation_probability",
        lat,
        lon
    );

    let response = client
        .get(&weather_url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(response)
}

fn print_location(geocode_response: &GeocodeResponse) {
    let display_name = geocode_response
        .display_name
        .as_ref()
        .unwrap_or(&"Unknown location".to_string())
        .yellow();
    println!("Местоположение: {}", display_name);
}

fn print_weather_data(weather_response: &WeatherResponse) {
    if let Some(current) = &weather_response.current {
        let current_temp = current.temperature_2m.unwrap_or(0.0);
        let temp_color = if current_temp > 0.0 {
            format!("{:.2}°C", current_temp).green()
        } else {
            format!("{:.2}°C", current_temp).blue()
        };

        let current_wind = current.wind_speed_10m.unwrap_or(0.0);
        let wind_color = if current_wind > 5.0 {
            format!("{:.2}M/C", current_wind).yellow()
        } else if current_wind > 15.0 {
            format!("{:.2}M/C", current_wind).red()
        } else {
            format!("{:.2}M/C", current_wind).green()
        };

        let current_prep = current.precipitation_probability.unwrap_or(0.0);
        let prep_color = if current_prep > 30.0 {
            format!("{:.2}%", current_prep).yellow()
        } else if current_prep > 60.0 {
            format!("{:.2}%", current_prep).red()
        } else {
            format!("{:.2}%", current_prep).green()
        };

        println!("Текущая температура: {}", temp_color);
        println!("Скорость ветра: {}", wind_color);
        println!("Вероятность осадков: {}", prep_color);
    } else {
        println!("Не удалось получить текущие данные о погоде.");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("Введите город: ");
    let city: String = read!();

    match fetch_geocode(&city).await {
        Ok(geocode_response) => {
            if let Some(location) = geocode_response.get(0) {
                print_location(location);

                let latitude = &location.lat;
                let longitude = &location.lon;

                match fetch_weather_data(latitude, longitude).await {
                    Ok(weather_response) => {
                        print_weather_data(&weather_response);
                    }
                    Err(err) => {
                        eprintln!("Ошибка при получении данных о погоде: {}", err);
                    }
                }
            } else {
                println!("Город не найден.");
            }
        }
        Err(err) => {
            eprintln!("Ошибка: {}", err);
        }
    }

    Ok(())
}
