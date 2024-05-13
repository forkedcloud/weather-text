use reqwest;
use serde_json;

pub async fn get(url: &str) ->  Result<(String, String), String> {
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let lat = match json.get("lat") {
        Some(lat) => lat.to_string(),
        None => return Err("error getting latitude".to_string())
    };
    let lon = match json.get("lon") {
        Some(lon) => lon.to_string(),
        None => return Err("error getting longitude".to_string())
    };

    return Ok((lat, lon))
}
