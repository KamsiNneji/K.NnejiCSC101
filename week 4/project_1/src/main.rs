use std::io;

fn main() {
    // Input values for a, b, and c
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = a.trim().parse().expect("Please type a number!");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = b.trim().parse().expect("Please type a number!");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: f64 = c.trim().parse().expect("Please type a number!");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root: {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}