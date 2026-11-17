use crate::{COMPOSITES, Context, Language, Script};

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn simple_languages() {
    let context = Context::default();
    let scripts = ok!(context.language("en"));
    assert!(scripts.contains(&Script {
        code: "Latn",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(scripts.iter().any(|script| script.code == "Brai"));
    assert!(context.language("xx").is_none());
}

#[test]
fn simple_scripts() {
    let context = Context::default();
    let languages = ok!(context.script("Latn"));
    assert!(languages.contains(&Language {
        code: "en",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(languages.iter().any(|language| language.code == "fr"));
    assert!(context.script("Xxxx").is_none());
}

#[test]
fn simple_case() {
    let context = Context::default();
    assert_eq!(context.language("EN"), context.language("en"));
    assert_eq!(context.language("FR-Gallo"), context.language("fr-gallo"));
    assert_eq!(context.script("latn"), context.script("Latn"));
    assert_eq!(context.script("LATN"), context.script("Latn"));
    // The codes are given as they are written in the standards.
    assert!(
        context
            .languages
            .values()
            .any(|(code, _)| *code == "fr-gallo")
    );
    assert!(context.scripts.values().any(|(code, _)| *code == "Latn"));
    assert!(context.scripts.values().all(|(code, _)| *code != "latn"));
}

#[test]
fn simple_consistency() {
    let context = Context::default();
    let forward: usize = context
        .languages
        .values()
        .map(|(_, scripts)| scripts.len())
        .sum();
    let backward: usize = context
        .scripts
        .values()
        .map(|(_, languages)| languages.len())
        .sum();
    assert_eq!(forward, backward);
    assert!(forward > 700);
}

#[test]
fn composite_languages() {
    let context = Context::default();
    let codes = |language: &str| {
        ok!(context.language(language))
            .iter()
            .map(|script| script.code)
            .collect::<Vec<_>>()
    };
    assert_eq!(codes("ko"), ["Hang", "Hani", "Kore"]);
    assert_eq!(codes("zh"), ["Hani", "Hans", "Hant", "Hebr"]);
}

#[test]
fn composite_scripts() {
    let context = Context::default();
    let find = |script: &str, language: &str| {
        ok!(context.script(script))
            .iter()
            .filter(|candidate| candidate.code == language)
            .cloned()
            .collect::<Vec<_>>()
    };

    // A combination is listed as is and under each of its scripts.
    let korean = find("Kore", "ko");
    assert_eq!(korean.len(), 1);
    assert_eq!(korean[0].grid, Some("은태"));
    assert_eq!(find("Hang", "ko"), korean);
    assert_eq!(find("Hani", "ko"), korean);

    // A sample given specifically for a script takes precedence.
    assert_eq!(find("Hani", "ja"), find("Jpan", "ja"));
    assert_eq!(find("Hira", "ja").len(), 1);
    assert_eq!(find("Hira", "ja")[0].grid, Some("にんげん"));
    assert_eq!(find("Kana", "ja").len(), 1);
    assert_eq!(find("Kana", "ja")[0].grid, Some("ニンゲン"));

    // The most populous variant wins.
    assert_eq!(find("Hani", "zh"), find("Hans", "zh"));
}

#[test]
fn composite_consistency() {
    let mut codes = COMPOSITES.iter().map(|(code, _)| *code).collect::<Vec<_>>();
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), COMPOSITES.len());
    for (code, scripts) in COMPOSITES {
        assert!(!scripts.is_empty());
        assert!(code.len() == 4 && scripts.iter().all(|script| script.len() == 4));
        // No chains: the scripts are not composites themselves.
        assert!(scripts.iter().all(|script| !codes.contains(script)));
    }
    // All of them are present in the data.
    let context = Context::default();
    assert!(codes.iter().all(|code| context.script(code).is_some()));
}
