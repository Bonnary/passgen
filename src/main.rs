use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author="Vet Bonnary", version="0.0.1", about="simple password generator in rust", long_about = None)]
struct Args {
    password_length: u32,
    /// password will have only lower case letters
    #[arg(short, long)]
    lower: bool,
    /// password will have only upper case letters
    #[arg(short, long)]
    upper: bool,
    /// password will have number
    #[arg(short, long)]
    number: bool,
    /// password will have symbols
    #[arg(short, long)]
    symbols: bool,
    /// password will have everything
    #[arg(short, long)]
    all: bool,
}
fn main() {
    let args = Args::parse();
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "@#$&_-()=%*:/!?+.";
    let mut passstring = String::new();
    if args.all {
        passstring = lower.to_owned() + upper + numbers + symbols;
    }
    if args.lower {
        passstring.push_str(lower);
    }
    if args.upper {
        passstring.push_str(upper);
    }
    if args.number {
        passstring.push_str(numbers);
    }
    if args.symbols {
        passstring.push_str(symbols);
    }

    let mut password = String::new();
    for _ in 0..args.password_length {
        let ran_num: u32 = rand::thread_rng()
            .gen_range(1..passstring.len())
            .try_into()
            .unwrap();
        password.push(passstring.chars().nth(ran_num.try_into().unwrap()).unwrap());
    }
    println!("{}", password);
}
