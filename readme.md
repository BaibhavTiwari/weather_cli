# Weather_cli

Welcome to **Weather_cli**, a simple Rust application that fetches and displays current weather information for a given city and country using the OpenWeatherMap API. The application provides data such as temperature, humidity, atmospheric pressure, and wind speed in a colored and formatted output for better readability.

## Features

- Fetches real-time weather data using the OpenWeatherMap API.
- Displays weather information such as temperature, humidity, pressure, and wind speed.
- Text coloring based on weather conditions for better visualization.
- Supports searching for multiple cities in a single session.
- Includes emoji indicators for temperature levels.

## Requirements

- **Rust** (Ensure Rust is installed by following [this guide](https://www.rust-lang.org/learn/get-started)).
- **OpenWeatherMap API Key** (Sign up at [OpenWeatherMap](https://openweathermap.org/) to get an API key).
- Internet connection to fetch live weather data.
- Put your api key here " `let api_key = "YOUR API KEY";`

## Dependencies

The following crates are used in this project:
- `reqwest` for making HTTP requests.
- `serde` and `serde_json` for JSON deserialization.
- `colored` for colored terminal output.

You can add these dependencies in your `Cargo.toml` file:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "2.0"
```

Here’s the requested section in `.md` format:

markdown
## Setup

1. Clone this repository or copy the code into your local directory.
2. In the root of your project, run the following command to initialize a new Rust project:
   ```bash
   cargo new weather_cli
   ```
3. Navigate into the project directory:
   ```bash
   cd weather_ci
   ```
4. Add the required dependencies to your `Cargo.toml` file (as shown above).
5. Replace the contents of `src/main.rs` with the provided code.
6. Get your OpenWeatherMap API key from [OpenWeatherMap](https://openweathermap.org/).

## Usage

1. Run the program with:
   ```bash
   cargo run
   ```
2. The application will prompt you to:
   - Enter a city name.
   - Enter the corresponding country code (e.g., `US` for the United States).
3. The program will display the current weather conditions for the entered city in a user-friendly and colored format.
4. You will be asked if you want to search for another city. Enter `yes` to search again, or any other key to exit.
``` 