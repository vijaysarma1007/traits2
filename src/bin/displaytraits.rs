use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "Apple ::: [Kind : {:?}, Price: {}]",
            self.kind, self.price
        )
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple."),
            Err(error) => eprintln!("Error deleting file : {error}"),
        }
    }
}

fn main() {
    let lunch = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.20,
    };

    println!("{}", lunch);
    println!("{}", dinner_snack);
}
