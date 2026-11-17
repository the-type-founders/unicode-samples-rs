//! Samples for Unicode languages and scripts.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

// The ISO 15924 codes that stand for a combination or a variant of scripts, together with the
// scripts they are made of. The Unicode Standard assigns characters only to the constituent
// scripts, which is hence what one finds when inspecting a font or a text. The list is limited to
// the codes present in the source data.
const COMPOSITES: &[(&str, &[&str])] = &[
    // Combinations
    ("Jpan", &["Hani", "Hira", "Kana"]),
    ("Kore", &["Hang", "Hani"]),
    // Variants
    ("Hans", &["Hani"]),
    ("Hant", &["Hani"]),
];

/// A context.
#[derive(Clone, Debug)]
pub struct Context {
    languages: BTreeMap<String, Vec<Sample>>,
    scripts: BTreeMap<String, Vec<Sample>>,
}

/// A sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sample {
    /// The BCP 47 tag of the language.
    pub language: &'static str,
    /// The ISO 15924 code of the script.
    pub script: &'static str,
    /// The number of speakers, if known.
    pub population: Option<usize>,
    /// A text for a grid, usually two glyphs.
    pub grid: Option<&'static str>,
    /// A text for a line, usually a phrase.
    pub line: Option<&'static str>,
}

impl Context {
    /// Find the samples of a language.
    ///
    /// The case of the code is ignored. The samples are sorted by script.
    pub fn language(&self, code: &str) -> Option<&[Sample]> {
        self.languages
            .get(&code.to_ascii_lowercase())
            .map(Vec::as_slice)
    }

    /// Find the samples of a script.
    ///
    /// The case of the code is ignored. The samples are sorted by language and then by script.
    ///
    /// Samples given for a combination or a variant of scripts, such as `Kore`, are also listed
    /// under the constituent scripts, such as `Hang` and `Hani`.
    pub fn script(&self, code: &str) -> Option<&[Sample]> {
        self.scripts
            .get(&code.to_ascii_lowercase())
            .map(Vec::as_slice)
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut languages: BTreeMap<_, Vec<_>> = Default::default();
        let mut scripts: BTreeMap<_, Vec<_>> = Default::default();
        for &(language, script, population, grid, line) in DATA {
            let sample = Sample {
                language,
                script,
                population,
                grid,
                line,
            };
            let constituents = COMPOSITES
                .iter()
                .find(|(code, _)| *code == script)
                .map(|(_, scripts)| *scripts)
                .unwrap_or_default();
            // The keys are in lowercase to ignore the case when looking up.
            languages
                .entry(language.to_ascii_lowercase())
                .or_default()
                .push(sample);
            for script in std::iter::once(&script).chain(constituents) {
                scripts
                    .entry(script.to_ascii_lowercase())
                    .or_default()
                    .push(sample);
            }
        }
        for samples in languages.values_mut() {
            samples.sort_by_key(|sample| sample.script);
        }
        for samples in scripts.values_mut() {
            samples.sort_by_key(|sample| (sample.language, sample.script));
        }
        Self { languages, scripts }
    }
}
