fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/get")?;
    println!("{:#?}", resp);

    Ok(())
}
