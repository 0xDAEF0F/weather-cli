use std::io;
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

    let opts: Vec<weather_api::Location> = resp.results.unwrap().into_iter().take(5).collect();

    println!("Please select the desired option.");

    for (i, location) in opts.iter().enumerate() {
        if location.admin1.is_none() {
            println!("{}) {}, {}", i + 1, location.name.clone(), location.country.clone());
        } else {
            println!("{}) {}, {} — {}", i + 1, location.name.clone(), location.country.clone(),
            location.admin1.clone().unwrap())
        }
    }

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let trimmed_input = input_string.trim();

    let num = match trimmed_input.parse::<usize>() {
        Ok(num) => {
            if num > 0 && num <= opts.len() {
                num - 1
            } else {
                eprintln!("Was not able to parse option. Try again.");
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("Was not able to parse option. Try again.");
            std::process::exit(1);
        }
    };

    let (latitude, longitude) = opts[num].get_coordinates();

    Ok(())
}
