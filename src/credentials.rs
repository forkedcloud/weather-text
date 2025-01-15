use std::env::var;

pub fn get(credentials: Option<String>) -> Result<(String, String, String), String> {
    let path = match credentials {
        Some(custom_path) => custom_path,
        None => var("HOME")
            .map(|home| format!("{}/.weather-text/key", home))
            .map_err(|_| {
                "Unable to determine the HOME directory. Specify the credentials file using the --credentials flag.".to_string()
            })?,
    };

    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read credentials from file '{path}': {e}"))?;
    let mut lines = text.lines();

    let first_line = lines.next().unwrap_or("US").to_string();

    let (country, zip) = if first_line.chars().next().unwrap_or('0').is_digit(10) {
        // first line is ZIP code, so we set the country to "US" (default) and store first line as zip
        ("US".to_string(), first_line)
    } else {
        // first line is country code, store first line as country and second line as zip
        let zip = lines.next().ok_or("Credentials file is missing the ZIP code.")?.to_string();
        (first_line, zip)
    };

    let key = lines.next().ok_or("Credentials file is missing the API key.")?.to_string();
    Ok((country, zip, key))
}