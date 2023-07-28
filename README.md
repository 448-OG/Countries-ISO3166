### countries-iso3166

[![Rust](https://github.com/448-OG/Countries-ISO3166/actions/workflows/rust.yml/badge.svg)](https://github.com/448-OG/Countries-ISO3166/actions/workflows/rust.yml)  ![crates.io](https://img.shields.io/crates/v/countries-iso3166.svg)[![Docs](https://docs.rs/countries-iso3166/badge.svg)](https://docs.rs/countries-iso3166)


This crate is used to convert between UTF-8 Strings and country codes for easier handling of country names in ISO 3166 format. It has no dependencies and `no_std` compatible. The country names are converted to country codes in rust because country names can contain special characters not common in English which can cause confusion.

The current implementation is only for ISO 3166-1 [https://en.wikipedia.org/wiki/ISO_3166-1](https://en.wikipedia.org/wiki/ISO_3166-1).

Planned implementation for ISO 3166-3 [https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)

#### LICENSE
This code is licensed under APACHE-2.0 or MIT license.

#### Code of Conduct
All contributions and communication must adhere to the Rust Code of Conduct