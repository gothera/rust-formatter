use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size: i32, unit: &str) -> Self {
        let bytes = match unit {
            "kb" => size * 1000,
            "mb" => size * 1_000_000,
            "gb" => size * 1_000_000_000,
            _ => size, // default assumed to be bytes
        };

        Sizes {
            bytes: format!("{:.0} bytes", bytes),
            kilobytes: format!("{:.2} kilobytes", bytes / 1000),
            megabytes: format!("{:.2} megabytes", bytes / 1_000_000),
            gigabytes: format!("{:.2} gigabytes", bytes / 1_000_000_000),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run \"<number> <unit>\"");
        return;
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input. Please use the format '<number> <unit>' (e.g., '24 mb').");
        return;
    }

    let size = parts[0].parse::<i32>();
    if size.is_err() {
        println!("Invalid size number. Ensure it's a valid floating point number.");
        return;
    }

    let unit = parts[1].to_lowercase();

    let sizes = Sizes::new(size.unwrap(), &unit);
    println!("{:?}", sizes);
}