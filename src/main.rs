use clap::{Parser, Subcommand};
use clap::builder::TypedValueParser;
use temperature_units::TempUnits;

mod temperature_units;
mod weather_api;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch the temperature from any city.
    Temperature {
        /// City to query temperature from.
        #[arg(name = "city")]
        city: String,

        /// Units to display the temperature in.
        #[arg(
            long, 
            short,
            default_value_t = TempUnits::Celsius,
            value_parser = clap::builder::PossibleValuesParser::new(["fahrenheit", "celsius", "kelvin"])
                .map(|s| s.parse::<TempUnits>().unwrap())
        )]
        unit: TempUnits,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli = Cli::parse();

    let (city, _unit) = match &cli.command {
        Some(Commands::Temperature { city, unit }) => {
            (city, unit)
        }
        None => panic!("unreachable"),
    };

    let resp: weather_api::ApiResponse = reqwest::blocking::get(
        format!("https://geocoding-api.open-meteo.com/v1/search?name={city}"),
    )?
    .json()?;

    if resp.results.is_none() {
        println!("No results for what you are looking for. Try again.");
        return Ok(())
    }

    for location in resp.results.unwrap().iter().take(5) {
        println!("name: {} — country: {} — admin1: {:?}",
        location.name, location.country, location.admin1);
    }

    Ok(())
}
