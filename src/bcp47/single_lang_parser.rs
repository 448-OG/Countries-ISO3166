use std::collections::HashMap;

use crate::{BC47LanguageInfo, CountriesIso31661Error, CountriesIso31661Result};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingleLanguageTranslationMap {
    pub bcp47_code: String,
    pub translations: HashMap<String, String>,
}

impl SingleLanguageTranslationMap {
    pub fn parse(source_path: &str, input: &str) -> CountriesIso31661Result<Self> {
        let lines = input.lines();
        let mut language = None;
        let mut translations = HashMap::new();

        let mut current_key: Option<String> = None;
        let mut multiline_value = String::new();
        let mut in_multiline = false;

        for line in lines {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // First non-empty line starting with # = language
            if language.is_none() && line.starts_with('#') {
                language = Some(line.trim_start_matches('#').trim().to_string());
                continue;
            }

            // Multiline continuation
            if in_multiline {
                multiline_value.push('\n');
                multiline_value.push_str(line);

                if line.ends_with('"') {
                    multiline_value.pop(); // remove trailing quote
                    if let Some(key) = current_key.take() {
                        translations.insert(key, multiline_value.clone());
                    }
                    multiline_value.clear();
                    in_multiline = false;
                }

                continue;
            }

            // Parse key = value
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let mut value = value.trim().to_string();

                if value.starts_with('"') {
                    value.remove(0); // remove leading quote
                    if value.ends_with('"') {
                        value.pop(); // remove trailing quote
                        translations.insert(key, value);
                    } else {
                        // multiline value starts
                        in_multiline = true;
                        current_key = Some(key);
                        multiline_value = value;
                    }
                } else {
                    translations.insert(key, value);
                }
            } else {
                return Err(CountriesIso31661Error::InvalidLanguageEntryParsed {
                    source_path: source_path.to_string(),
                    line: line.to_string(),
                });
            }
        }

        let bcp47_code = language.ok_or(CountriesIso31661Error::LanguageBcp47CodeNotFound(
            source_path.to_string(),
        ))?;

        let parsed_code: BC47LanguageInfo = bcp47_code.as_str().into();

        if parsed_code == BC47LanguageInfo::UnsupportedLanguage {
            return Err(CountriesIso31661Error::UnsupportedBcp47Code {
                source_path: source_path.to_string(),
                invalid_lang: bcp47_code,
            });
        }

        Ok(Self {
            bcp47_code,
            translations,
        })
    }

    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn bcp47_code(&self) -> &str {
        self.bcp47_code.as_str()
    }

    pub fn translations(&self) -> &HashMap<String, String> {
        &self.translations
    }

    pub fn translations_owned(&self) -> Vec<(String, String)> {
        self.translations
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::SingleLanguageTranslationMap;

    #[test]
    fn valid_lang() {
        let source_contents = include_str!("../../example_data/test-single-lang.bcp47");
        let source_path = "../../example_data/test-single-lang.bcp47";

        let parse = SingleLanguageTranslationMap::parse(source_path, source_contents);

        assert!(parse.is_ok());
    }

    #[test]
    fn invalid_lang() {
        const LANG: &str = r#"""
        hello_world = hello world
        lorem = "Lorem ipsum dolor sit amet consectetur adipisicing elit. 
        Fuga impedit porro possimus quo obcaecati molestias perferendis, consectetur iure natus.
        At ipsa laudantium iusto illo fuga tempora facilis. Vero, tempora libero."
        """#;

        let parse = SingleLanguageTranslationMap::parse("static str", LANG);

        assert!(parse.is_err());
    }
}
