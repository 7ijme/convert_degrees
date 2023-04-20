use bpaf::Bpaf;
use convert_degrees::{ConvertTemperatures, DegreeUnit, Temperature};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(short, long, argument("from"), fallback(DegreeUnit::Celsius))]
    /// From which unit to convert -- Can be celsius(c), fahrenheit(f) kelvin(k), degrees(d) or radians(r)
    from: DegreeUnit,
    #[bpaf(short, long, argument("to"), fallback(DegreeUnit::Fahrenheit))]
    /// To which unit to convert   -- Can be celsius(c), fahrenheit(f) kelvin(k), degrees(d) or radians(r)
    to: DegreeUnit,
    #[bpaf(positional("degrees"))]
    degrees: f64,
}

fn main() {
    let args: Args = args().run();

    let temp = Temperature {
        value: args.degrees,
        unit: args.from.clone(),
    };

    println!(
        "{} = {}",
        temp,
        temp.to(&args.to),
    );
}
