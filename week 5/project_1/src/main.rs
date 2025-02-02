use std::io::{self, Write};

fn main() {
    // Menu items and prices
    let menu = [
        ('P', "Poundo Yam/Edinkaiko Soup", 3200),
        ('F', "Fried Rice & Chicken", 3000),
        ('A', "Amala & Ewedu Soup", 2500),
        ('E', "Eba & Egusi Soup", 2000),
        ('W', "White Rice & Stew", 2500),
    ];

    // Display menu
    println!("Menu:");
    for (code, item, price) in menu.iter() {
        println!("{} = {}: N{}", code, item, price);
    }

    // Input order
    let mut total_cost = 0;
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        println!("\nEnter the type of food (P, F, A, E, W) or 'done' to finish:");
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        let food_type = buffer.trim().to_uppercase();

        if food_type == "DONE" {
            break;
        }

        let item = menu.iter().find(|(code, _, _)| code.to_string() == food_type);
        if let Some((_, _, price)) = item {
            println!("Enter the quantity:");
            buffer.clear();
            stdin.read_line(&mut buffer).expect("Failed to read line");
            let quantity: u32 = buffer.trim().parse().expect("Please type a number!");

            total_cost += price * quantity;
        } else {
            println!("Invalid food type, please try again.");
        }
    }

    // Apply discount if applicable
    if total_cost > 10_000 {
        total_cost = (total_cost as f64 * 0.95) as i32;
        println!("A 5% discount has been applied.");
    }

    // Display total charges
    println!("\nTotal charges: N{}", total_cost);
}