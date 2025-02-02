struct Laptop {
    brand: String,
    quantity: i32,
    unit_cost: i32,
}

impl Laptop {
    fn total_cost(&self) -> i32 {
        self.quantity * self.unit_cost
    }

    fn customer_purchase_cost(&self, purchase_quantity: i32) -> i32 {
        purchase_quantity * self.unit_cost
    }
}

fn main() {
    let laptops = vec![
        Laptop {
            brand: "HP".to_string(),
            quantity: 10,
            unit_cost: 650000,
        },
        Laptop {
            brand: "IBM".to_string(),
            quantity: 6,
            unit_cost: 550000,
        },
        Laptop {
            brand: "Toshiba".to_string(),
            quantity: 10,
            unit_cost: 755000,
        },
        Laptop {
            brand: "Dell".to_string(),
            quantity: 4,
            unit_cost: 850000,
        },
    ];

    let total_cost: i32 = laptops.iter().map(|laptop| laptop.total_cost()).sum();
    println!("Total cost of the consignment: {} Naira", total_cost);

    let purchase_quantity = 3;
    let total_customer_cost: i32 = laptops
        .iter()
        .map(|laptop| laptop.customer_purchase_cost(purchase_quantity))
        .sum();
    println!(
        "Total cost if a customer purchases {} laptops from each brand: {} Naira",
        purchase_quantity, total_customer_cost
    );
}