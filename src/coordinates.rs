use reqwest;
use serde_json;


pub async fn get(url: &str) -> Result<(String, String), String> {
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let text = response.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let latitude = json.get("lat")
        .ok_or("Error: Latitude not found in response")?
        .to_string();
    let longitude = json.get("lon")
        .ok_or("Error: Longitude not found in response")?
        .to_string();

    Ok((latitude, longitude))
}
