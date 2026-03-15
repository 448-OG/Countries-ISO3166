pub type CountriesIso31661Result<T> = Result<T, CountriesIso31661Error>;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CountriesIso31661Error {
    #[cfg(feature = "std")]
    #[error("Invalid Entry")]
    InvalidLanguageEntryParsed { source_path: String, line: String },
    #[cfg(feature = "std")]
    #[error("The Identifier `{0}` to lookup the translation was not found")]
    IdentifierNotFound(String),
    #[cfg(feature = "std")]
    #[error("The BCP-47 code `{bcp47_code}` for Identifier `{identifier}` was not found")]
    Bcp47EntryNotFound {
        identifier: String,
        bcp47_code: String,
    },
    #[cfg(feature = "std")]
    #[error("Encountered and invalid BCP-47 code `{invalid_lang}` while parsing the file `{source_path}`")]
    UnsupportedBcp47Code {
        source_path: String,
        invalid_lang: String,
    },
}
