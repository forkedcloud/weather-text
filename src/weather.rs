pub async fn get(lat: String, lon: String, units: String, format: String, key: String) -> Result<String, String> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units={units}&appid={key}");

    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let text = response.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let mut result: String = Default::default();
    for element in format.split(" ") {
        let substring: String;

        if element.starts_with('%') {
            substring = match &element[0..2] {
                "%T" => {
                    let temperature = &json["main"]["temp"].to_string().parse::<f32>().unwrap();
                    let accuracy = &element.split(":").last().unwrap().parse::<usize>().unwrap();
                    let unit = match units.as_str() {
                        "standard" => "K",
                        "metric" => "°C",
                        "imperial" => "°F",
                        _ => {
                            // clap validates this argument so this shouldn't happen
                            return Err(format!("unknown unit {units}"));
                        }
                    };
                    format!("{:.1$}{unit}", temperature, accuracy)
                }
                "%D" => {
                    let description = &json["weather"][0]["description"].to_string();
                    format!("{}", description.trim_matches('"'))
                }
                "%I" => {
                    let icon = get_icon(&json["weather"][0]["icon"].as_str().unwrap());
                    format!("{icon}")
                }
                _ => {
                    return Err(format!("unknown format identifier {element}"));
                }
            };
        } else {
            substring = element.to_string();
        }

        result.push_str(&substring);
        result.push_str(" ");
    }

    return Ok(result);
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
