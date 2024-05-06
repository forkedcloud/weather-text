use std::env::var;

pub fn get(credentials: String) -> Result<(String, String), String> {
    let path: String;
    if credentials == "default" {
        path = format!("{}/.weather-text/key", var("HOME").unwrap());
    } else {
        path = credentials;
    }

    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();

    let zip = lines.next().unwrap();
    let key = lines.next().unwrap();

    return Ok((zip.to_string(), key.to_string()))
}
