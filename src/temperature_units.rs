#[derive(Clone)]
pub enum TempUnits {
    Fahrenheit,
    Celsius,
}

impl std::fmt::Display for TempUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TempUnits::Celsius => "celsius",
            TempUnits::Fahrenheit => "fahrenheit",
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
            _ => Err(format!("Unknown units: {s}")),
        }
    }
}
