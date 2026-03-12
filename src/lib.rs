#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![no_std]

mod iso_3166_1;
pub use iso_3166_1::*;

#[cfg(feature = "bcp47")]
mod bcp47_language_info;
#[cfg(feature = "bcp47")]
pub use bcp47_language_info::*;

#[cfg(test)]
mod sanity_checks {
    use crate::*;

    #[test]
    fn no_special_characters() {
        {
            let country = "Kenya";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::KE);
        }

        {
            let country = "Mauritius";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::MU);
        }

        {
            let country = "Afghanistan";
            let code: CountryIso31661 = country.into();
            assert!(code == CountryIso31661::AF)
        }

        {
            let country = "AF";
            let code: CountryIso31661 = country.into();
            assert!(code == CountryIso31661::AF)
        }
    }

    #[test]
    fn with_special_characters() {
        {
            let country = "Virgin Islands, U.S.";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::VI);
        }

        {
            let country = "Cocos (Keeling) Islands";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::CC);
        }

        {
            let country = "Côte d'Ivoire";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::CI);
        }

        {
            let country = "Timor-Leste";
            let country_code_iso_31661: CountryIso31661 = country.into();

            let into_str = country_code_iso_31661.as_str();
            assert_eq!(into_str, country);

            assert_eq!(country_code_iso_31661, CountryIso31661::TL);
        }
    }
}
