//! Samples for Unicode languages and scripts.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

/// A context.
#[derive(Clone, Copy, Debug, Default)]
pub struct Context;

/// A language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Language {
    /// The ISO 639 code in lowercase.
    pub code: &'static str,
    /// A sample for a grid, usually two glyphs.
    pub grid: Option<&'static str>,
    /// A sample for a line, usually a phrase.
    pub line: Option<&'static str>,
}

/// A script.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Script {
    /// The ISO 15924 code in lowercase.
    pub code: &'static str,
    /// A sample for a grid, usually two glyphs.
    pub grid: Option<&'static str>,
    /// A sample for a line, usually a phrase.
    pub line: Option<&'static str>,
}

impl Context {
    /// Group the samples by language.
    ///
    /// The keys are language codes, and the values are the scripts the languages are written in.
    pub fn languages() -> BTreeMap<&'static str, Vec<Script>> {
        let mut languages: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, _, grid, line) in DATA.iter().cloned() {
            languages.entry(language).or_default().push(Script {
                code: script,
                grid,
                line,
            });
        }
        languages
    }

    /// Group the samples by script.
    ///
    /// The keys are script codes, and the values are the languages written in the scripts.
    pub fn scripts() -> BTreeMap<&'static str, Vec<Language>> {
        let mut scripts: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, _, grid, line) in DATA.iter().cloned() {
            scripts.entry(script).or_default().push(Language {
                code: language,
                grid,
                line,
            });
        }
        scripts
    }
}
