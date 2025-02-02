use std::io::{self, Write};

fn main() {
    loop {
        // Display options to the user
        println!("Select a geometric calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        // Read the user's choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please try again.");
                continue;
            }
        };

        match choice {
            1 => {
                // Area of Trapezium
                let (base1, base2, height) = get_trapezium_inputs();
                let area = 0.5 * (base1 + base2) * height;
                println!("Area of Trapezium: {}", area);
            }
            2 => {
                // Area of Rhombus
                let (diagonal1, diagonal2) = get_rhombus_inputs();
                let area = 0.5 * diagonal1 * diagonal2;
                println!("Area of Rhombus: {}", area);
            }
            3 => {
                // Area of Parallelogram
                let (base, altitude) = get_parallelogram_inputs();
                let area = base * altitude;
                println!("Area of Parallelogram: {}", area);
            }
            4 => {
                // Area of Cube
                let side = get_cube_input();
                let area = 6.0 * side.powi(2);
                println!("Area of Cube: {}", area);
            }
            5 => {
                // Volume of Cylinder
                let (radius, height) = get_cylinder_inputs();
                let volume = std::f64::consts::PI * radius.powi(2) * height;
                println!("Volume of Cylinder: {}", volume);
            }
            6 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_trapezium_inputs() -> (f64, f64, f64) {
    let base1 = read_input("Enter base1: ");
    let base2 = read_input("Enter base2: ");
    let height = read_input("Enter height: ");
    (base1, base2, height)
}

fn get_rhombus_inputs() -> (f64, f64) {
    let diagonal1 = read_input("Enter diagonal1: ");
    let diagonal2 = read_input("Enter diagonal2: ");
    (diagonal1, diagonal2)
}

fn get_parallelogram_inputs() -> (f64, f64) {
    let base = read_input("Enter base: ");
    let altitude = read_input("Enter altitude: ");
    (base, altitude)
}

fn get_cube_input() -> f64 {
    read_input("Enter the length of the side: ")
}

fn get_cylinder_inputs() -> (f64, f64) {
    let radius = read_input("Enter radius: ");
    let height = read_input("Enter height: ");
    (radius, height)
}

fn read_input(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}
