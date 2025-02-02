fn main() {
    // Principal amount (P)
    let principal: f64 = 520_000_000.0;

    // Annual interest rate (R) in percentage
    let rate: f64 = 10.0;

    // Number of years (n)
    let years: u32 = 5;

    // Calculating compound interest
    let amount = principal * (1.0 + rate / 100.0).powi(years as i32);
    let compound_interest = amount - principal;

    // Printing the result
    println!("Compound Interest for {} years at {}% per annum is: N{:.2}", years, rate, compound_interest);
}