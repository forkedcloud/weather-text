# weather-text

This is a simple little program to fetch weather data from OpenWeatherMap and
print the temprature and an icon. It was originally built to be used in a Wibar
widget, but it can be used anywhere else you want.

## installation

Clone the repository, build it, and install with
```
cargo install --path .
```
To be used in a Wibar widget, you'll want it to be in your PATH. I recommend 
manually copying the binary into a directory such as `/usr/bin/`, as awesomewm
doesn't seem to be able to find anything in `~/.cargo`.

## formatting

`weather-text` now supports formatting using the following identifiers:

* `%I` - weather icon
* `%T:_` - temperature with specified precision
* `%D` - weather description

For example, the format string `$I $T:0 $D` will produce an output such as
```
 41°F light rain
```
More identifiers will be added eventually.

## api key

This tool utilizes the OpenWeatherMap API, so you'll need to go create an 
acccount at https://openweathermap.org and generate an API key. Once you have
this key, create the file `~/.weather-text/key`; this is the only place the
tool will check because I'm too lazy to make this location configurable.

The file should have your ZIP code on the first line, and your API key on the
second line. For example,
```
10024
6e71b3cac15d32fe2d36c270887df9479c25c640
```
Once this file is in place, the tool should work. 

If you find something you don't like or want to improve upon this, you're more
than welcome to open a pull request. I wrote this in like 20 minutes to make a
wibar widget so it doesn't have the highest quality standards.
