use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DegreeUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for DegreeUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" | "c" => Ok(DegreeUnit::Celsius),
            "fahrenheit" | "f" => Ok(DegreeUnit::Fahrenheit),
            "kelvin" | "k" => Ok(DegreeUnit::Kelvin),
            _ => Err("Invalid unit"),
        }
    }
}

impl std::fmt::Display for DegreeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DegreeUnit::Celsius => write!(f, "°C"),
            DegreeUnit::Fahrenheit => write!(f, "°F"),
            DegreeUnit::Kelvin => write!(f, "K"),
        }
    }
}
