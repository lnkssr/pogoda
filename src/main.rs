use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct IpApiResponse {
    city: Option<String>,
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    country: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: Option<CurrentWeather>,
    hourly: Hourly,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature_2m: Option<f64>,
    wind_speed_10m: Option<f64>,
    precipitation_probability: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct Hourly {
    temperature_2m: Vec<f64>,
    relative_humidity_2m: Vec<f64>,
    wind_speed_10m: Vec<f64>,
    precipitation_probability: Vec<f64>,
}

async fn fetch_ip_info(url: &str) -> Result<IpApiResponse, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?
        .json::<IpApiResponse>()
        .await?;
    Ok(response)
}

async fn fetch_weather_data(url: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;
    Ok(response)
}

fn print_location(ip_info: &IpApiResponse) {
    let city = ip_info.city.as_deref().unwrap_or("Unknown city");
    let region = ip_info.region_name.as_deref().unwrap_or("Unknown region");
    let country = ip_info.country.as_deref().unwrap_or("Unknown country");
    println!("Местоположение: {} ({}, {})", city, region, country);
}

fn print_weather_data(weather_response: &WeatherResponse) {
    // Вывод текущих значений погоды
    if let Some(current) = &weather_response.current {
        println!(
            "Текущая температура: {:.2}°C",
            current.temperature_2m.unwrap_or(0.0)
        );
        println!(
            "Скорость ветра: {:.2} м/с",
            current.wind_speed_10m.unwrap_or(0.0)
        );
        println!(
            "Вероятность осадков: {:.2}%",
            current.precipitation_probability.unwrap_or(0.0)
        );
    } else {
        println!("Не удалось получить текущие данные о погоде.");
    }

    // Найти последний индекс для часовых данных
    let last_index = weather_response
        .hourly
        .temperature_2m
        .len()
        .saturating_sub(1);

    // Последние значения
    let last_temperature = weather_response
        .hourly
        .temperature_2m
        .get(last_index)
        .unwrap_or(&0.0);
    let last_humidity = weather_response
        .hourly
        .relative_humidity_2m
        .get(last_index)
        .unwrap_or(&0.0);
    let last_wind_speed = weather_response
        .hourly
        .wind_speed_10m
        .get(last_index)
        .unwrap_or(&0.0);
    let last_precipitation = weather_response
        .hourly
        .precipitation_probability
        .get(last_index)
        .unwrap_or(&0.0);

    // Вывод последних значений погоды
    println!("Последнее значение температуры: {:.2}°C", last_temperature);
    println!("Последнее значение влажности: {:.2}%", last_humidity);
    println!(
        "Последнее значение скорости ветра: {:.2} м/с",
        last_wind_speed
    );
    println!("Последняя вероятность осадков: {:.2}%", last_precipitation);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const IP_INFO_URL: &str = "http://ip-api.com/json";
    let ip_info = fetch_ip_info(IP_INFO_URL).await?;

    // Проверяем наличие координат
    let latitude = ip_info.lat.ok_or("Latitude not found")?;
    let longitude = ip_info.lon.ok_or("Longitude not found")?;

    // URL для получения данных о погоде
    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m,precipitation_probability&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m,precipitation_probability",
        latitude,
        longitude
    );

    let weather_response = fetch_weather_data(&weather_url).await?;

    print_location(&ip_info);
    print_weather_data(&weather_response);

    Ok(())
}
