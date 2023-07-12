

use std::fmt;

enum TicketType {
    Backstage { name: String, price: u32 },
    VIP { name: String, price: u32 },
    Standard { price: u32 },
}

impl fmt::Display for TicketType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TicketType::Backstage { name, price } => write!(f, "Backstage ticket for {} (price: {})", name, price),
            TicketType::VIP { name, price } => write!(f, "VIP ticket for {} (price: {})", name, price),
            TicketType::Standard { price } => write!(f, "Standard ticket (price: {})", price),
        }
    }
}

fn main() {
    let tickets = vec![
        TicketType::Backstage { name: "John Doe".to_string(), price: 1000 },
        TicketType::VIP { name: "Jane Doe".to_string(), price: 500 },
        TicketType::Standard { price: 200 },
    ];

    for ticket in tickets {
        println!("{}", ticket);
    }
}


