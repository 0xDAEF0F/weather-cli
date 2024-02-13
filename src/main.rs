use clap::{Parser, Subcommand};
use clap::builder::TypedValueParser;
use temperature_units::TempUnits;

mod temperature_units;

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


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Temperature { city, unit }) => {
            println!("city: {city} — units: {unit}");
        }
        None => {}
    }

    // Continued program logic goes here...
}
