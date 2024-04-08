use std::env::var;

pub fn get() -> Result<(String, String), String> {
    let path = format!("{}/.weather-text/key", var("HOME").unwrap());
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();

    let zip = lines.next().unwrap();
    let key = lines.next().unwrap();

    return Ok((zip.to_string(), key.to_string()))
}
