//! Samples for Unicode languages and scripts.

#![allow(clippy::type_complexity)]

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

include!(concat!(env!("OUT_DIR"), "/data.rs"));

// The ISO 15924 codes that stand for a combination or a variant of scripts, together with the
// scripts they are made of. The Unicode Standard assigns characters only to the constituent
// scripts, which is hence what one finds when inspecting a font or a text. The list is limited to
// the codes present in the source data.
const COMPOSITES: &[(&str, &[&str])] = &[
    // Combinations
    ("jpan", &["hani", "hira", "kana"]),
    ("kore", &["hang", "hani"]),
    // Variants
    ("hans", &["hani"]),
    ("hant", &["hani"]),
];

/// A context.
#[derive(Clone, Copy, Debug, Default)]
pub struct Context;

/// A language.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Language {
    /// The ISO 639 code in lowercase.
    pub code: &'static str,
    /// The number of speakers, if known.
    pub population: Option<usize>,
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
    /// The number of speakers, if known.
    pub population: Option<usize>,
    /// A sample for a grid, usually two glyphs.
    pub grid: Option<&'static str>,
    /// A sample for a line, usually a phrase.
    pub line: Option<&'static str>,
}

type Entry = (
    &'static str,
    &'static str,
    Option<usize>,
    Option<&'static str>,
    Option<&'static str>,
);

impl Context {
    /// Group the samples by language.
    ///
    /// The keys are language codes, and the values are the scripts the languages are written in,
    /// sorted by population in descending order with ties broken by code.
    ///
    /// Samples given for a combination or a variant of scripts, such as `kore`, are also listed
    /// under the constituent scripts, such as `hang` and `hani`, unless the language has a sample
    /// of its own there.
    pub fn languages() -> BTreeMap<&'static str, Vec<Script>> {
        let mut languages: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, population, grid, line) in entries() {
            languages.entry(language).or_default().push(Script {
                code: script,
                population,
                grid,
                line,
            });
        }
        for scripts in languages.values_mut() {
            sort(scripts, |script| (script.population, script.code));
        }
        languages
    }

    /// Group the samples by script.
    ///
    /// The keys are script codes, and the values are the languages written in the scripts, sorted
    /// by population in descending order with ties broken by code.
    ///
    /// Samples given for a combination or a variant of scripts, such as `kore`, are also listed
    /// under the constituent scripts, such as `hang` and `hani`, unless the language has a sample
    /// of its own there.
    pub fn scripts() -> BTreeMap<&'static str, Vec<Language>> {
        let mut scripts: BTreeMap<_, Vec<_>> = Default::default();
        for (language, script, population, grid, line) in entries() {
            scripts.entry(script).or_default().push(Language {
                code: language,
                population,
                grid,
                line,
            });
        }
        for languages in scripts.values_mut() {
            sort(languages, |language| (language.population, language.code));
        }
        scripts
    }
}

fn entries() -> Vec<Entry> {
    let mut entries = DATA.to_vec();
    let mut seen = entries
        .iter()
        .map(|(language, script, ..)| (*language, *script))
        .collect::<BTreeSet<_>>();
    // When several composites of a language lead to the same script, such as `hans` and `hant`
    // leading to `hani`, the most populous one wins, with ties broken by code.
    let mut composites = DATA
        .iter()
        .filter_map(|entry| {
            COMPOSITES
                .iter()
                .find(|(code, _)| *code == entry.1)
                .map(|(_, scripts)| (entry, *scripts))
        })
        .collect::<Vec<_>>();
    composites.sort_by(|(one, _), (other, _)| {
        other
            .2
            .unwrap_or(0)
            .cmp(&one.2.unwrap_or(0))
            .then_with(|| one.1.cmp(other.1))
    });
    for (&(language, _, population, grid, line), scripts) in composites {
        for script in scripts {
            // The samples given specifically for the language and script take precedence.
            if seen.insert((language, script)) {
                entries.push((language, script, population, grid, line));
            }
        }
    }
    entries
}

fn sort<T, F>(values: &mut [T], key: F)
where
    F: Fn(&T) -> (Option<usize>, &'static str),
{
    values.sort_by(|first, second| {
        let (one, first) = key(first);
        let (other, second) = key(second);
        other
            .unwrap_or(0)
            .cmp(&one.unwrap_or(0))
            .then_with(|| first.cmp(second))
    });
}
