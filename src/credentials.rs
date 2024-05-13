use std::env::var;

pub fn get(credentials: Option<String>) -> Result<(String, String), String> {
    let path = match credentials {
        Some(path) => path,
        None => format!("{}/.weather-text/key", var("HOME").unwrap())
    };

    let text = std::fs::read_to_string(path).map_err(|e| format!("error reading credentials: {e}"))?;
    let mut lines = text.lines();

    let zip = match lines.next() {
        Some(zip) => zip,
        None => return Err("credentials missing zip code".to_string())
    };
    let key = match lines.next() {
        Some(key) => key,
        None => return Err("credentials missing api key".to_string())
    };

    return Ok((zip.to_string(), key.to_string()))
}
