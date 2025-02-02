fn main() {
    let initial_value: f64 = 510000.0;
    let depreciation_rate: f64 = 5.0;
    let years: i32 = 3;

    let depreciated_value = calculate_depreciation(initial_value, depreciation_rate, years);

    println!("The value of the TV after {} years is: N{:.2}", years, depreciated_value);
}

fn calculate_depreciation(p: f64, r: f64, n: i32) -> f64 {
    p * (1.0 - r / 100.0).powi(n)
}