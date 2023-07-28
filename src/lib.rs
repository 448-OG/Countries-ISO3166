#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![no_std]

mod iso_3166_1;
pub use iso_3166_1::*;

#[cfg(test)]
mod sanity_checks {
    use crate::*;
    #[test]
    fn no_special_characters() {
        {
            let country = "Kenya";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::KE);
        }

        {
            let country: &str = "Mauritius";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::MU);
        }
    }

    #[test]
    fn with_special_characters() {
        {
            let country = "Virgin Islands, U.S.";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::VI);
        }

        {
            let country = "Cocos (Keeling) Islands";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::CC);
        }

        {
            let country = "Côte d'Ivoire";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::CI);
        }

        {
            let country = "Timor-Leste";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str: &str = country_code_iso_31661.into();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::TL);
        }
    }
}
