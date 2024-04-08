// preface: some of the error handling in this program may seem horrible, but
// it's designed so that the program should *not* panic. any errors should be 
// printed to standard out so that they can be displayed in the wibar

use tokio;

mod credentials;
mod coordinates;
mod weather;

#[tokio::main]
async fn main() {
    let (zip, key) = match credentials::get() {
        Ok(credentials) => credentials,
        Err(e) => {
            println!("{e}");
            return
        }
    };

    let url = format!("http://api.openweathermap.org/geo/1.0/zip?zip={zip},US&appid={key}");

    let (lat, lon) = match coordinates::get(&url).await {
        Ok(coordinates) => coordinates,
        Err(e) => {
            println!("{e}");
            return 
        }
    };

    let (temp, weather) = match weather::get(lat, lon, key).await {
        Ok(result) => result,
        Err(e) => {
            println!("{e}");
            return
        }
    };

    println!("{temp} {weather}")
}
