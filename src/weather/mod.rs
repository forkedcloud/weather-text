pub async fn get(lat: String, lon: String, key: String) -> Result<(String, String, String), String> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units=imperial&appid={key}");
    let response = match reqwest::get(url).await {
        Ok(response) => response,
        Err(e) => return Err(e.to_string())
    };

    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(e.to_string())
    };

    let json: serde_json::Value = match serde_json::from_str(&text) {
        Ok(json) => json,
        Err(e) => return Err(e.to_string())
    };

    let temp = &json["main"]["temp"].to_string();
    let temp = format!("{}°F", temp.split('.').nth(0).unwrap());

    let description = &json["weather"][0]["description"].to_string();
    let description = description.trim_matches('"');

    let icon = match json["weather"][0]["icon"].as_str().unwrap() {
        "01d" => "",
        "01n" => "",
        "02d" => "",
        "02n" => "",
        "03d" => "",
        "03n" => "",
        "04d" => "",
        "04n" => "",
        "09d" => "",
        "09n" => "",
        "10d" => "",
        "10n" => "",
        "11d" => "",
        "11n" => "",
        "13d" => "",
        "13n" => "",
        "50d" => "",
        "50n" => "",
        _ => ""
    };

    return Ok((temp, description.to_string(), icon.to_string()))
}
