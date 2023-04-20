use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum DegreeUnit {
    Celsius,
    Fahrenheit,
    Kelvin,

    Degrees,
    Radians,
}

impl FromStr for DegreeUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "celsius" | "c" => Ok(DegreeUnit::Celsius),
            "fahrenheit" | "f" => Ok(DegreeUnit::Fahrenheit),
            "kelvin" | "k" => Ok(DegreeUnit::Kelvin),
            "degrees" | "deg" | "°" | "d" => Ok(DegreeUnit::Degrees),
            "radians" | "rad" | "r" => Ok(DegreeUnit::Radians),
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
            DegreeUnit::Degrees => write!(f, "°"),
            DegreeUnit::Radians => write!(f, "rad"),
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

pub struct Temperature {
    pub value: f64,
    pub unit: DegreeUnit,
}

pub trait ConvertTemperatures {
    fn to_celsius(&self) -> f64;
    fn to_fahrenheit(&self) -> f64;
    fn to_kelvin(&self) -> f64;
    fn to_degrees(&self) -> f64;
    fn to_radians(&self) -> f64;
    fn to(&self, unit: DegreeUnit) -> Temperature {
        match unit {
            DegreeUnit::Celsius => Temperature {
                value: self.to_celsius(),
                unit: DegreeUnit::Celsius,
            },
            DegreeUnit::Fahrenheit => Temperature {
                value: self.to_fahrenheit(),
                unit: DegreeUnit::Fahrenheit,
            },
            DegreeUnit::Kelvin => Temperature {
                value: self.to_kelvin(),
                unit: DegreeUnit::Kelvin,
            },
            DegreeUnit::Degrees => Temperature {
                value: self.to_degrees(),
                unit: DegreeUnit::Degrees,
            },
            DegreeUnit::Radians => Temperature {
                value: self.to_radians(),
                unit: DegreeUnit::Radians,
            },
        }
    }
}

impl ConvertTemperatures for Temperature {
    fn to_celsius(&self) -> f64 {
        match self.unit {
            DegreeUnit::Celsius => self.value,
            DegreeUnit::Fahrenheit => (self.value - 32.0) * 5.0 / 9.0,
            DegreeUnit::Kelvin => self.value - 273.15,
            DegreeUnit::Degrees => self.value * std::f64::consts::PI / 180.0,
            DegreeUnit::Radians => self.value * 180.0 / std::f64::consts::PI,
        }
    }

    fn to_fahrenheit(&self) -> f64 {
        match self.unit {
            DegreeUnit::Celsius => self.value * 9.0 / 5.0 + 32.0,
            DegreeUnit::Fahrenheit => self.value,
            DegreeUnit::Kelvin => self.value * 9.0 / 5.0 - 459.67,
            DegreeUnit::Degrees => self.value * std::f64::consts::PI / 180.0 * 9.0 / 5.0 + 32.0,
            DegreeUnit::Radians => self.value * 180.0 / std::f64::consts::PI * 9.0 / 5.0 + 32.0,
        }
    }

    fn to_kelvin(&self) -> f64 {
        match self.unit {
            DegreeUnit::Celsius => self.value + 273.15,
            DegreeUnit::Fahrenheit => (self.value + 459.67) * 5.0 / 9.0,
            DegreeUnit::Kelvin => self.value,
            DegreeUnit::Degrees => self.value * std::f64::consts::PI / 180.0 + 273.15,
            DegreeUnit::Radians => self.value * 180.0 / std::f64::consts::PI + 273.15,
        }
    }

    fn to_degrees(&self) -> f64 {
        match self.unit {
            DegreeUnit::Celsius => self.value * 180.0 / std::f64::consts::PI,
            DegreeUnit::Fahrenheit => {
                (self.value - 32.0) * 5.0 / 9.0 * 180.0 / std::f64::consts::PI
            }
            DegreeUnit::Kelvin => self.value - 273.15 * 180.0 / std::f64::consts::PI,
            DegreeUnit::Degrees => self.value,
            DegreeUnit::Radians => self.value * 180.0 / std::f64::consts::PI,
        }
    }

    fn to_radians(&self) -> f64 {
        match self.unit {
            DegreeUnit::Celsius => self.value * std::f64::consts::PI / 180.0,
            DegreeUnit::Fahrenheit => {
                (self.value - 32.0) * 5.0 / 9.0 * std::f64::consts::PI / 180.0
            }
            DegreeUnit::Kelvin => self.value - 273.15 * std::f64::consts::PI / 180.0,
            DegreeUnit::Degrees => self.value * std::f64::consts::PI / 180.0,
            DegreeUnit::Radians => self.value,
        }
    }
}
