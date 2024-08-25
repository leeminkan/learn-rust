// use reqwest;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let response = reqwest::blocking::get("https://www.axs.orxxg")?;

//     println!("Status: {}", response.status());
//     println!("Headers:\n{:#?}", response.headers());

//     let body = response.text()?;
//     println!("Body:\n{}", body);

//     Ok(())
// }

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
