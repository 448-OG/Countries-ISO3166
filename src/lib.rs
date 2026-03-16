#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod iso_3166_1;
pub use iso_3166_1::*;

mod errors;
pub use errors::*;

#[cfg(feature = "std")]
mod bcp47;
#[cfg(feature = "std")]
pub use bcp47::*;

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
