// Preface: some of the error handling in this program may seem horrible, but
// it's designed so that the program should *not* panic. any errors should be 
// printed to standard out so that they can be displayed in the wibar

use tokio;
use clap::Parser;

mod cli;
mod credentials;
mod coordinates;
mod weather;

#[tokio::main]
async fn main() {
    let cli = cli::CLI::parse();

    let (country_code, zip, key) = match credentials::get(cli.credentials) {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("Error: {e}");
            eprintln!("For more information, try '--help'.");
            return;
        }
    };

    let geo_url = format!("http://api.openweathermap.org/geo/1.0/zip?zip={zip},{country_code}&appid={key}");

    let (latitude, longitude) = match coordinates::get(&geo_url).await {
        Ok(coordinates) => coordinates,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    

    let weather_result = match weather::get(latitude, longitude, cli.units, cli.format, key).await {
        Ok(result) => result,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    println!("{weather_result}");
}
