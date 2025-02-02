fn main() {
    let commissioners = vec![
        "Abogunrin Abambi Daqujud",
        "Mufutau Afeez Benjodi",
        "Adeyemo Timothy Abogun",
        "Osazuwa Faith Efejese",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense & Security",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
    ];

    println!("Consolidated Dataset:");
    for i in 0..commissioners.len() {
        println!(
            "Name: {}, Ministry: {}, Geopolitical Zone: {}",
            commissioners[i], ministries[i], geopolitical_zones[i]
        );
    }
}