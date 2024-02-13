use std::path::PathBuf;
use clap::{Parser, Subcommand};
use clap::builder::TypedValueParser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone)]
enum TempUnits {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl std::fmt::Display for TempUnits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                TempUnits::Celsius => "celsius",
                TempUnits::Fahrenheit => "fahrenheit",
                TempUnits::Kelvin => "kelvin"
            };
            s.fmt(f)
        }
    }

impl std::str::FromStr for TempUnits {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "fahrenheit" => Ok(Self::Fahrenheit),
                "celsius" => Ok(Self::Celsius),
                "kelvin" => Ok(Self::Kelvin),
                _ => Err(format!("Unknown units: {s}")),
            }
        }
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

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Temperature { city, unit }) => {
            println!("city: {city} — units: {unit}");
        }
        None => {}
    }

    // Continued program logic goes here...
}
