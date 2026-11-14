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
    /// The keys are language codes, and the values are the scripts the languages are written in,
    /// sorted by population in descending order with ties broken by code.
    pub fn languages() -> BTreeMap<&'static str, Vec<Script>> {
        let mut languages: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, population, grid, line) in DATA.iter().cloned() {
            languages.entry(language).or_default().push((
                population.unwrap_or(0),
                Script {
                    code: script,
                    grid,
                    line,
                },
            ));
        }
        languages
            .into_iter()
            .map(|(code, scripts)| (code, sort(scripts, |script| script.code)))
            .collect()
    }

    /// Group the samples by script.
    ///
    /// The keys are script codes, and the values are the languages written in the scripts, sorted
    /// by population in descending order with ties broken by code.
    pub fn scripts() -> BTreeMap<&'static str, Vec<Language>> {
        let mut scripts: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, population, grid, line) in DATA.iter().cloned() {
            scripts.entry(script).or_default().push((
                population.unwrap_or(0),
                Language {
                    code: language,
                    grid,
                    line,
                },
            ));
        }
        scripts
            .into_iter()
            .map(|(code, languages)| (code, sort(languages, |language| language.code)))
            .collect()
    }
}

fn sort<T, F>(mut values: Vec<(u32, T)>, code: F) -> Vec<T>
where
    F: Fn(&T) -> &'static str,
{
    values.sort_by(|(one, first), (other, second)| {
        other.cmp(one).then_with(|| code(first).cmp(code(second)))
    });
    values.into_iter().map(|(_, value)| value).collect()
}
