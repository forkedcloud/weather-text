pub async fn get(
    latitude: String,
    longitude: String,
    units: String,
    format: String,
    key: String,
) -> Result<String, String> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={latitude}&lon={longitude}&units={units}&appid={key}");

    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let text = response.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let mut result = String::new();
    for element in format.split_whitespace() {
        let substring = if element.starts_with('%') {
            match &element[0..2] {
                "%T" => {
                    let temp = json["main"]["temp"]
                        .as_f64()
                        .ok_or("Error parsing temperature")?;
                    let precision = element.split(':').nth(1).unwrap_or("0").parse::<usize>().unwrap_or(0);
                    let unit = match units.as_str() {
                        "standard" => "K",
                        "metric" => "°C",
                        "imperial" => "°F",
                        // Clap validates this argument so this shouldn't happen.
                        _ => return Err(format!("Unknown unit: {units}")),
                    };
                    format!("{:.precision$}{unit}", temp)
                }
                "%D" => json["weather"][0]["description"]
                    .as_str()
                    .unwrap_or("Unknown description")
                    .to_string(),
                "%I" => get_icon(json["weather"][0]["icon"].as_str().unwrap_or("")).to_string(),
                _ => return Err(format!("Unknown format identifier: {element}")),
            }
        } else {
            element.to_string()
        };
        result.push_str(&substring);
        result.push(' ');
    }

    Ok(result.trim_end().to_string())
}

fn get_icon(code: &str) -> &str {
    return match code {
        "01d" => "",
        "01n" => "",
        "02d" => "",
        "02n" => "",
        "03d" => "",
        "03n" => "",
        "04d" => "",
        "04n" => "",
        "09d" => "",
        "09n" => "",
        "10d" => "",
        "10n" => "",
        "11d" => "",
        "11n" => "",
        "13d" => "",
        "13n" => "",
        "50d" => "",
        "50n" => "",
        _ => ""
    };
}
