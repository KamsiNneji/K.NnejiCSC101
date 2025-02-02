use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let categories = vec![
        ("Lager", vec!["Star", "Heineken", "Goldberg", "33 Export", "Desperados"]),
        ("Stout", vec!["Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    let mut file = File::create("high_quality_drinks.txt")?;

    for (category, brands) in categories {
        writeln!(file, "{}:", category)?;
        for brand in brands {
            writeln!(file, "- {}", brand)?;
        }
    }

    println!("File 'high_quality_drinks.txt' created successfully!");

    Ok(())
}