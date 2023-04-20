# Convert Degrees

With this small project you can convert between Celsius, Fahrenheit and Kelvin & Degrees and radians very easily.

---------------------------------------------------

Bin Installation:
```bash
cargo install convert_degrees --features bin
```

Usage:
```bash
deg [-f from] [-t to] <degrees> 

Available options:
    -f, --from <from>  From which unit to convert -- Can be celsius(c), fahrenheit(f) kelvin(k),
                       degrees(d) or radians(r)
    -t, --to <to>      To which unit to convert   -- Can be celsius(c), fahrenheit(f) kelvin(k),
                       degrees(d) or radians(r)
    -h, --help         Prints help information
```

----------------------------------------------------

Library installation
```bash
cargo add convert_degrees
```

```rust
use convert_degrees::{ConvertTemperatures, DegreeUnit, Temperature};

fn main() {
    let celcius = Temperature {
        value: 12f64,
        unit: DegreeUnit::Celsius,
    };

    let fahrenheit = celcius.to(DegreeUnit::Fahrenheit);

    println!("{} is the same as {}", celcius, fahrenheit);
    // output: 12°C is the same as 53.6°F
}
```
