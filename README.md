# 🌤️ weather-text
**A simple program which fetches some weather data from OpenWeatherMap and displays a temperature with its icon.**

Originally designed to be used within a Wibar widget, it's flexible enough to fit where you need it! 

**⚠️ NOTICE:**
With recent changes, the output icons now rely on Nerd Font. Ensure you're using a compatible font.

## 🚀 Installation
Clone the repository:
bash
```
git clone https://github.com/joepigott/weather-text.git
```

Build and install with:
bash
```
cargo install --path .
```
Make sure that cargo (a build system and package manager for Rust) is installed in your system.

**🛠️ To use in a Wibar widget**:
Make sure the binary is in your PATH. It is highly recommended to manually copy it to a directory like `/usr/bin/`, since AwesomeWM may not detect files in `~/.cargo`.

## 🖋️ Formatting Options
`weather-text` supports customizable formatting using the following identifiers:
* `%I` - Weather icon
* `%T:_` - Temperature with specified precision
* `%D` - Weather description
  
### 🧩 Example Formats
1️⃣ **Format string**: `%I %T:0 %D`

**💬 Output**:
```
 41°F light rain
```
2️⃣ **Format String**:
```
good morning. the weather is %D with a temperature of %T:0
```
**💬 Output**:
```
good morning. the weather is overcast clouds with a temperature of 61°F
```
✨ **Note**: More identifiers will be added in future updates (if possible).

## 🔑 API Key Setup
This tool uses the OpenWeatherMap API. To get started:
1. **Create an account** at [OpenWeatherMap](https://openweathermap.org/).
2. **Generate an API Key**.
3. Save the key in a file at ``~/.weather-text/key`` or specify a custom path with the `--credentials` flag when running the program.

### 🔐 Key File Format Example:
**1️⃣ Format 1: Default Country (US) 🇺🇸**
```
10024
6e71b3cac15d32fe2d36c270887df9479c25c640
```
* **Line 1**: Your **ZIP code** 🏠 (e.g., `10024`).
* **Line 2**: Your **API key** 🔑 (e.g., `6e71b3cac15d32fe2d36c270887df9479c25c640`).
* If no **country code** is provided, the tool will **automatically assume** the **United States (US) 🇺🇸** as your country.

**2️⃣ Format 2: Custom Country 🌍**
```
CA
T9W
6e71b3cac15d32fe2d36c270887df9479c25c640
```
* **Line 1**: Your **country code** 🌎 (e.g., `CA` for Canada 🍁, PK for Pakistan 🇵🇰, IN for India 🇮🇳, etc.).
* **Line 2**: Your **ZIP code** 🏠 (e.g., `T9W`).
* **Line 3**: Your **API key** 🔑 (e.g., `6e71b3cac15d32fe2d36c270887df9479c25c640`).
* With this format, you can **specify your country** 🌍. The tool will read the country code, ZIP code, and API key, and use them accordingly.

### ⚙️ How it works:
* The program will check if the first line of configuration file starts with **numbers**, if it does it will assume that its the **ZIP code** that is provided and will default to the **United States (US) 🇺🇸**.
* Else it will use the **provided country code** 🌎 on the first line, the second line as **ZIP code** 🏠 and third as **API key** 🔑.
   
Once your key file is ready, you're all set to 🚀 **use the tool!**

---
## 💡 Contributions Welcome
Found something you'd like to improve? 🤔

Feel free to open a **pull request**!

🛠️ **Note**: This was initially built in just 20 minutes for a Wibar widget, so there's room for improvement.
