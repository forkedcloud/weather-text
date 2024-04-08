use reqwest;
use serde_json;

pub async fn get(url: &str) ->  Result<(String, String), String> {
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

    let lat = match json.get("lat") {
        Some(lat) => lat,
        None => return Err("missing latitude!".to_string())
    };
    let lon = match json.get("lon") {
        Some(lon) => lon,
        None => return Err("missing longitude!".to_string())
    };

    return Ok((lat.to_string(), lon.to_string()))
}
