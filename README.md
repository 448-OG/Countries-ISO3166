### countries-iso3166

[![Rust](https://github.com/448-OG/Countries-ISO3166/actions/workflows/rust.yml/badge.svg)](https://github.com/448-OG/Countries-ISO3166/actions/workflows/rust.yml) ![crates.io](https://img.shields.io/crates/v/countries-iso3166.svg)[![Docs](https://docs.rs/countries-iso3166/badge.svg)](https://docs.rs/countries-iso3166)

This crate is used to convert between UTF-8 Strings and country codes for easier handling of country names in ISO 3166 format and handling language translation parsing for BCP-47 codes. It has no dependencies and `no_std` compatible but `no_std` disabled BCP47 language translations parsing. The country names are converted to country codes in rust because country names can contain special characters not common in English which can cause confusion.

The current implementation is only for ISO 3166-1 [https://en.wikipedia.org/wiki/ISO_3166-1](https://en.wikipedia.org/wiki/ISO_3166-1).

Planned implementation for ISO 3166-3 [https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)

### Examples parsing country names and codes

```rust
use countries_iso3166::CountryIso31661;

// Initializing the `CountryIso31661` using a code
let country = CountryIso31661::KE;

// Getting the name for the country
country.as_str();

//Getting the code for the country
country.country_code();

//
// -- Convertions from a &str --
//

// Converting from a country's name
let country = "Kenya";
let country_code_iso_31661: CountryIso31661 = country.into();

let into_str = country_code_iso_31661.as_str();
assert_eq!(into_str, country);

assert_eq!(country_code_iso_31661, CountryIso31661::KE);


// Converting from a country's code
let country_code = "KE";
let code: CountryIso31661 = country.into();
assert!(code == CountryIso31661::KE);

// Countries with special characters are also supported if the characters are UTF-8
let country = "Côte d'Ivoire";
let country_code_iso_31661: CountryIso31661 = country.into();

let into_str = country_code_iso_31661.as_str();
assert_eq!(into_str, country);

assert_eq!(country_code_iso_31661, CountryIso31661::CI);

```

### Examples parsing country names and codes

This is can be a single language file or multi-language file.

#### Single language file

The single language file only contains only the translations for a single language.
See [The BCP47 file extension below for the formats of these files](#the-bcp-47-file).

```rust
use countries_iso3166::SingleLanguageTranslationMap;

let source_contents = include_str!("../test-single-lang.bcp47");
let source_path = "../test-single-lang.bcp47";

let parse = SingleLanguageTranslationMap::parse(source_path, source_contents);

assert!(parse.is_ok());


const LANG: &str = r#"""
hello_world = hello world
lorem = "Lorem ipsum dolor sit amet consectetur adipisicing elit.
Fuga impedit porro possimus quo obcaecati molestias perferendis, consectetur iure natus.
At ipsa laudantium iusto illo fuga tempora facilis. Vero, tempora libero."
"""#;

let parse = SingleLanguageTranslationMap::parse("static str", LANG);

assert!(parse.is_err());
```

#### Multi-language file

```rust
use countries_iso3166::{CountriesIso31661Error, MultiLanguageTranslationMap};

let source_contents = include_str!("../test-lang.bcp47");
let source_path = "../test-lang.bcp47";

assert!(MultiLanguageTranslationMap::new(source_path, source_contents).is_ok());

let source_contents = include_str!("../test-lang-invalid.bcp47");
let source_path = "../test-lang-invalid.bcp47";

assert_eq!(
    MultiLanguageTranslationMap::new(source_path, source_contents).err(),
    Some(CountriesIso31661Error::UnsupportedBcp47Code {
        source_path: source_path.to_string(),
        invalid_lang: "en-USS".to_string()
    })
);
```

### The BCP-47 file

As seen above the language translations format can be for a single language or multi-language.
This data interchange file format is very simple for average folk to translate to their native language without the learning curve of other formats like JSON, YAML, TOML, etc or modify the binary to add languages.

#### The Sigle-Language File format

The BCP-47 translation format is done with the following syntax:

- A `# ` describing the BCP-47 case sensitive language code.
- A new line with the `identifier` of the language entry followed by an `=` sign and lastly the translation. If the translation is multi-line, the start of the multi-line is defined by a starting quote mark `"`, words spanning multiple lines and ending with another quotation mark `"`.

```sh
# en-US
identifier = translation
identifier = "multi-line
translation
can
span multiple lines
"
```

Example:

```sh
# en-US
hello_world_button = hello world
lorem = "Lorem ipsum dolor sit amet consectetur adipisicing elit.
Fuga impedit porro possimus quo obcaecati molestias perferendis, consectetur iure natus.
 At ipsa laudantium iusto illo fuga tempora facilis. Vero, tempora libero."
```

#### The Multi-Language File format

The BCP-47 translation format for a multiline is done with the following syntax:

- A `# ` describing the opening of an identifier. This identifier is used to lookup th translations to other languages
- A new line with the `BCP-47` code of the language followed by an `=` sign and lastly the translation. If the translation is multi-line, the start of the multi-line is defined by a starting quote mark `"`, words spanning multiple lines and ending with another quotation mark `"`.

```sh
# identifier text
bcp47-code = translation
bcp47-code = "multi-line
translation
can
span multiple lines
"
```

Example:

```sh
# hello world
sw = Hujambo Dunia
en-US =  Hello World
zh-Hans = 你好，世界

# Lorem Text
sw = Hujambo Dunia
en-US = "This info spans multiple lines.
Fuga impedit porro possimus quo obcaecati molestias perferendis, consectetur iure natus.
 At ipsa laudantium iusto illo fuga tempora facilis. Vero, tempora libero."
zh-Hans = 你好，世界
#
```

If reading from a file like in the examples, it is recommended to name the file with a `.bcp47` file extension so that in future if a parser is made for this you can get syntaxt highlighting as seen in the example `../test-lang.bcp47` file.

### LICENSE

This code is licensed under APACHE-2.0 or MIT license.

### Code of Conduct

All contributions and communication must adhere to the Rust Code of Conduct
