fn main() {
    // Sales data
    let items = ["Toothpaste", "Soap", "Cream", "Oil", "Juice"];
    let quantities = [10, 20, 15, 10, 5];
    let amounts = [500_000.0, 1_000_000.0, 250_000.0, 800_000.0, 2_000_000.0];

    // Calculate total amount
    let total_amount: f64 = amounts.iter().sum();
    
    // Calculate total quantity
    let total_quantity: i32 = quantities.iter().sum();
    
    // Calculate average amount per item
    let average_amount = total_amount / items.len() as f64;

    // Calculate average quantity per item
    let average_quantity = total_quantity as f64 / items.len() as f64;

    // Print the results
    println!("Total Amount: N{:.2}", total_amount);
    println!("Average Amount per Item: N{:.2}", average_amount);
    println!("Total Quantity: {}", total_quantity);
    println!("Average Quantity per Item: {:.2}", average_quantity);
}