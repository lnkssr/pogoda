
# Weather and IP Information Fetcher

This project is a simple Rust application that retrieves the current weather information and IP-based geolocation data. It uses the `ip-api.com` API to determine the user's location and the `open-meteo.com` API to fetch weather data based on that location.

## Features

- Fetches geolocation information (city, region, country) based on the user's public IP.
- Fetches weather data for the user's location, including:
  - Current temperature
  - Wind speed
  - Precipitation probability
  - Hourly data such as temperature, humidity, wind speed, and precipitation probability.

## Dependencies

- [tokio](https://crates.io/crates/tokio): For asynchronous operations.
- [reqwest](https://crates.io/crates/reqwest): HTTP client for making requests.
- [serde](https://crates.io/crates/serde): For serializing and deserializing JSON data.
- [serde_json](https://crates.io/crates/serde_json): For parsing JSON responses.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

2. Clone the repository:

   ```bash
   git clone repos.name
   ```

3. Change into the project directory:

   ```bash
   cd weather-ip-fetcher
   ```

4. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

Simply run the compiled program:

```bash
cargo run --release
```

The application will automatically retrieve the IP information and fetch weather data for the geolocation based on your public IP.

### Example Output:

```
Location: New York (New York, United States)
Current temperature: 24.5°C
Wind speed: 3.20 m/s
Precipitation probability: 0.00%

Last hourly data:
Temperature: 22.7°C
Humidity: 70.00%
Wind speed: 2.70 m/s
Precipitation probability: 10.00%
```

## How It Works

1. The program sends a request to `ip-api.com` to retrieve geolocation information (city, region, country) based on the user's IP address.
2. Once the location is retrieved (latitude and longitude), the program constructs a request URL for the `open-meteo.com` API.
3. The weather data is fetched, and the current and hourly weather information is printed to the console.

## API Endpoints

- **IP Info**: [ip-api.com](http://ip-api.com/json)
- **Weather**: [open-meteo.com](https://open-meteo.com/en)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contribution

Feel free to fork this repository and submit pull requests. Contributions are welcome!

