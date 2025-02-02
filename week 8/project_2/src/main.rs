// Struct to represent a Developer with name and years of experience
#[derive(Debug)]
struct Developer {
    name: String,
    years_of_experience: i32,
}

// Function to find the Developer with the highest years of experience
fn find_highest_experience(devs: Vec<Developer>) -> Developer {
    let mut max_experience_dev = &devs[0];
    
    for dev in &devs {
        if dev.years_of_experience > max_experience_dev.years_of_experience {
            max_experience_dev = dev;
        }
    }
    
    max_experience_dev.clone()
}

fn main() {
    // Sample list of developers
    let developers = vec![
        Developer { name: "Alice".to_string(), years_of_experience: 5 },
        Developer { name: "Bob".to_string(), years_of_experience: 10 },
        Developer { name: "Carol".to_string(), years_of_experience: 7 },
        Developer { name: "Dave".to_string(), years_of_experience: 3 },
        Developer { name: "Eve".to_string(), years_of_experience: 8 },
    ];
    
    // Find the developer with the highest years of experience
    let max_dev = find