# 🧼 Sopje

> 🚧 This package is a WIP

Sopje is a simple SOAP client builder written for Rust

## Usage

Create a struct that makes use of the `sopje::sdk` attribute, passing in the location of the WSDL.

Then you can call `::new()` on your SDK and start using it!

```rs
#[sopje::sdk("http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso")]
struct Sdk;

fn main() {
    let client = Sdk::new();

    client.some_call().unwrap();
}
```

## The name

"Sopje" literally translates to "little soap" from Dutch and is typically used to refer to a cleaning solution.
