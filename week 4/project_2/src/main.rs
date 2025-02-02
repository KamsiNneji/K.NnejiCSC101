use std::io;

fn main() {
    // Input values for experience and age
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience = experience.trim().eq_ignore_ascii_case("yes");

    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: u32 = age.trim().parse().expect("Please type a number!");

    // Calculate the annual incentive
    let incentive = calculate_incentive(experience, age);
    println!("The annual incentive is: N{}", incentive);
}

fn calculate_incentive(experience: bool, age: u32) -> u32 {
    if experience {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12 // Monthly incentive converted to annual
        } else {
            0 // No specific incentive mentioned for this age range
        }
    } else {
        100_000
    }
}