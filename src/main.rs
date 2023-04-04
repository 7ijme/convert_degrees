use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    #[bpaf(short('f'), long("fahrenheit"))]
    /// Whether to convert from fahrenheit instead of celsius
    fahrenheit: bool,
    #[bpaf(positional("degrees"))]
    degrees: f64,
}

fn main() {
    let args: Args = args().run();

    let degrees = if args.fahrenheit {
        (args.degrees - 32.0) * 5.0 / 9.0
    } else {
        args.degrees * 9.0 / 5.0 + 32.0
    };

    let unit = if args.fahrenheit { "°C" } else { "°F" };

    println!("{}{}", degrees, unit);
}
