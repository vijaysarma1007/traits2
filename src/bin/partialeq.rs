//#[derive(PartialEq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("New York", "London", "8:00");
    let b = Flight::new("New York", "London", "12:00");
    let c = Flight::new("New York", "Los Angeles", "8:00");
    println!("{}", a == b);
    println!("{}", b == c);
    println!("{}", a.eq(&b));
    println!("{}", b.eq(&c));
    println!("{}", b.ne(&c));

    let d = BusTrip::new("los Angeles", "Tokyo", "8:00");
    println!("bustrip: {}", a == d);
    //println!("bustrip: {}", d==a);

    // println!("{}", 3 == 3);
}
