use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: CurrentWeather,
    hourly: Hourly,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: f64,
    wind_speed_10m: f64,
}

#[derive(Deserialize, Debug)]
struct Hourly {
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    time: Vec<String>,
}

async fn fetch_weather_data(url: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.json::<WeatherResponse>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=55&longitude=38&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m";

    match fetch_weather_data(url).await {
        Ok(weather_response) => {
            // Вывод текущих значений
            println!("Текущая температура: {:.2}°C", weather_response.current.temperature_2m);
            println!("Текущая скорость ветра: {:.2} м/с", weather_response.current.wind_speed_10m);

            // Найти последний индекс для часовых данных
            let last_index = weather_response.hourly.time.len().saturating_sub(1);

            // Последние значения
            let last_temperature = weather_response.hourly.temperature_2m.get(last_index).unwrap_or(&0.0);
            let last_humidity = weather_response.hourly.relative_humidity_2m.get(last_index).unwrap_or(&0.0);
            let last_wind_speed = weather_response.hourly.wind_speed_10m.get(last_index).unwrap_or(&0.0);

            println!("Последнее значение температуры: {:.2}°C", last_temperature);
            println!("Последнее значение влажности: {:.2}%", last_humidity);
            println!("Последнее значение скорости ветра: {:.2} м/с", last_wind_speed);
        }
        Err(e) => {
            eprintln!("Ошибка при запросе данных о погоде: {}", e);
        }
    }

    Ok(())
}
