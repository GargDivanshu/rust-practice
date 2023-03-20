fn main() {
    let url = "https://finance.yahoo.com/quote/ICICIBANK.NS";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();
    print!("{}",body)
}
