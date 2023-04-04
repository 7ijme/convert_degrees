use std::f64::consts::PI;

use bpaf::Bpaf;
use degree::DegreeUnit;

mod degree;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(
        short,
        long,
        argument("from"),
        fallback(DegreeUnit::Celsius),
    )]
    /// From which unit to convert -- Can be celsius(c), fahrenheit(f) or kelvin(k)
    from: DegreeUnit,
    #[bpaf(
        short,
        long,
        argument("to"),
        fallback(DegreeUnit::Fahrenheit),
    )]
    /// To which unit to convert   -- Can be celsius(c), fahrenheit(f) or kelvin(k)
    to: DegreeUnit,
    #[bpaf(positional("degrees"))]
    degrees: f64,
}

fn main() {
    let args: Args = args().run();

    let (degrees, to): (f64, DegreeUnit) = match args.from {
        DegreeUnit::Celsius => match args.to {
            DegreeUnit::Kelvin => (args.degrees + 273.15, args.to),
            _ => {
                (args.degrees * 1.8 + 32.0, DegreeUnit::Fahrenheit)
            }
        },
        DegreeUnit::Fahrenheit => match args.to {
            DegreeUnit::Kelvin => ((args.degrees + 459.67) * 5.0 / 9.0, args.to),
            _ => {
                ((args.degrees - 32.0) / 1.8, DegreeUnit::Celsius)
            }
        },
        DegreeUnit::Kelvin => match args.to {

            DegreeUnit::Fahrenheit => (args.degrees * 9.0 / 5.0 - 459.67, args.to),
            _ => {
                (args.degrees - 273.15, DegreeUnit::Celsius)
            }
        },
        DegreeUnit::Degrees => match args.to {
            _ => {
                (args.degrees * PI / 180.0 , DegreeUnit::Radians)
            }
        },
        DegreeUnit::Radians => match args.to {
            _ => {
                (args.degrees * 180.0 / PI, DegreeUnit::Degrees)
            }
        },
    };

    println!("{}{} = {}{}", args.degrees, args.from, degrees, to);
}
