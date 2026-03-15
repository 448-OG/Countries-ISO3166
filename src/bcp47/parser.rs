use rapidhash::{HashMapExt, RapidHashMap as HashMap};

use crate::{CountriesIso31661Error, CountriesIso31661Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TranslationMap {
    // Some sentences may be long so having fixed size lookup seems like a good idea.
    // The hashing is done on the client to offload work from the server.
    // The server just sends the hashes of the `en-US` word or sentence.
    // Blake3 (in SIMD mode) is since it is very collision resistant compared to
    // non-cryptographic hashes which are faster but since `Krill-Server` is supposed
    // to be low maintenance, the cryptographic hash was used so users don't get confused
    // from collisions especially if sentences are large.
    // A hashmap is also used here in case users rearrange the translations JSON5 file.
    identifier_index: HashMap<blake3::Hash, usize>,
    // The key here is not hashed because the size is between 2 and less than 10 characters (BCP47).
    bcp47_index: HashMap<String, usize>,
    translations: Vec<Vec<String>>,
}

impl TranslationMap {
    pub fn new(source_path: &str, source_contents: &str) -> CountriesIso31661Result<Self> {
        let data = Self::parse(source_path, source_contents)?;

        let mut identifier_index = HashMap::<blake3::Hash, usize>::with_capacity(805);
        let mut bcp47_index = HashMap::<String, usize>::with_capacity(805);
        let mut translations = Vec::<Vec<String>>::default();

        data.into_iter()
            .enumerate()
            .for_each(|(identifier_index_key, (identifier, languages))| {
                let mut languages_inner = Vec::<String>::default();

                languages
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, (bcp47_code, translation))| {
                        languages_inner.push(translation);
                        bcp47_index.insert(bcp47_code.to_lowercase(), index);
                    });

                translations.push(languages_inner);

                identifier_index.insert(
                    blake3::hash(identifier.to_lowercase().as_bytes()),
                    identifier_index_key,
                );
            });

        Ok(Self {
            identifier_index,
            bcp47_index,
            translations,
        })
    }

    /// `Identifier` and `BCP-47 Code` are case sensitive
    pub fn get_translation(
        &self,
        identifier: &str,
        bcp47_code: &str,
    ) -> CountriesIso31661Result<String> {
        self.identifier_index
            .get(&blake3::hash(identifier.as_bytes()))
            .map(|identifier_index| {
                let bcp47_index = self.bcp47_index.get(bcp47_code).ok_or(
                    CountriesIso31661Error::Bcp47EntryNotFound {
                        identifier: identifier.to_string(),
                        bcp47_code: bcp47_code.to_string(),
                    },
                )?;
                let outcome = self.translations[*identifier_index][*bcp47_index].clone();

                Ok::<String, CountriesIso31661Error>(outcome)
            })
            .transpose()?
            .ok_or(CountriesIso31661Error::IdentifierNotFound(
                identifier.to_string(),
            ))
    }

    pub fn identifier_index(&self) -> &HashMap<blake3::Hash, usize> {
        &self.identifier_index
    }

    pub fn bcp47_index(&self) -> &HashMap<String, usize> {
        &self.bcp47_index
    }

    pub fn translations(&self) -> &[Vec<String>] {
        self.translations.as_slice()
    }

    /// The start of a sentence must have a space `# `
    pub fn parse(
        source_path: &str,
        input: &str,
    ) -> CountriesIso31661Result<HashMap<String, Vec<(String, String)>>> {
        let mut sentences: HashMap<String, Vec<(String, String)>> = HashMap::new();

        let mut current_sentence: Option<String> = None;
        let mut multiline_lang: Option<String> = None;
        let mut multiline_buffer = String::new();

        for raw_line in input.lines() {
            let line = raw_line.trim();

            if line.is_empty() {
                continue;
            }

            // multiline continuation
            if let Some(lang) = &multiline_lang {
                if line.ends_with('"') {
                    multiline_buffer.push_str(line.trim_end_matches('"'));

                    if let Some(sentence) = &current_sentence {
                        sentences
                            .get_mut(sentence)
                            .ok_or(CountriesIso31661Error::InvalidLanguageEntryParsed {
                                source_path: source_path.to_string(),
                                line: line.to_string(),
                            })?
                            .push((lang.clone(), multiline_buffer.clone()));
                    }

                    multiline_buffer.clear();
                    multiline_lang = None;
                } else {
                    multiline_buffer.push_str(line);
                    multiline_buffer.push('\n');
                }

                continue;
            }

            // new sentence
            if line.starts_with("# ") {
                let sentence = line.trim_start_matches("# ").to_string();
                sentences.entry(sentence.clone()).or_default();
                current_sentence = Some(sentence);
                continue;
            }

            // translation entry
            if let Some(eq_pos) = line.find('=') {
                let lang = line[..eq_pos].trim().to_string();

                let check_bc47_is_valid: crate::BC47LanguageInfo = lang.as_str().into();

                if check_bc47_is_valid == crate::BC47LanguageInfo::UnsupportedLanguage {
                    return Err(CountriesIso31661Error::UnsupportedBcp47Code {
                        source_path: source_path.to_string(),
                        invalid_lang: lang,
                    });
                }

                let value = line[eq_pos + 1..].trim();

                if value.starts_with('"') {
                    let content = value.trim_start_matches('"');

                    if content.ends_with('"') {
                        let final_value = content.trim_end_matches('"');

                        if let Some(sentence) = &current_sentence {
                            sentences
                                .get_mut(sentence)
                                .ok_or(CountriesIso31661Error::InvalidLanguageEntryParsed {
                                    source_path: source_path.to_string(),
                                    line: line.to_string(),
                                })?
                                .push((lang, final_value.to_string()));
                        }
                    } else {
                        multiline_lang = Some(lang);
                        multiline_buffer.push_str(content);
                        multiline_buffer.push('\n');
                    }
                } else {
                    if let Some(sentence) = &current_sentence {
                        sentences
                            .get_mut(sentence)
                            .ok_or(CountriesIso31661Error::InvalidLanguageEntryParsed {
                                source_path: source_path.to_string(),
                                line: line.to_string(),
                            })?
                            .push((lang, value.to_string()));
                    }
                }
            }
        }

        Ok(sentences)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Translation {
    bcp47: String,
    native: String,
}

#[cfg(test)]
mod sanity_checks {
    use crate::{CountriesIso31661Error, TranslationMap};

    #[test]
    fn parse_correct_translations() {
        let source_contents = include_str!("../../test-lang.bcp47");
        let source_path = "../../test-lang.bcp47";

        assert!(TranslationMap::new(source_path, source_contents).is_ok());
    }

    #[test]
    fn parse_incorrect_translations() {
        let source_contents = include_str!("../../test-lang-invalid.bcp47");
        let source_path = "../../test-lang-invalid.bcp47";

        assert_eq!(
            TranslationMap::new(source_path, source_contents).err(),
            Some(CountriesIso31661Error::UnsupportedBcp47Code {
                source_path: source_path.to_string(),
                invalid_lang: "en-USS".to_string()
            })
        );
    }
}
