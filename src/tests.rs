use crate::{COMPOSITES, Context, Sample};

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[test]
fn simple_languages() {
    let context = Context::default();
    let samples = ok!(context.language("en"));
    assert!(samples.contains(&Sample {
        language: "en",
        script: "Latn",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(samples.iter().any(|sample| sample.script == "Brai"));
    assert!(samples.iter().all(|sample| sample.language == "en"));
    assert!(context.language("xx").is_none());
}

#[test]
fn simple_scripts() {
    let context = Context::default();
    let samples = ok!(context.script("Latn"));
    assert!(samples.contains(&Sample {
        language: "en",
        script: "Latn",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(samples.iter().any(|sample| sample.language == "fr"));
    assert!(samples.iter().all(|sample| sample.script == "Latn"));
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
    let samples = ok!(context.language("FR-Gallo"));
    assert!(samples.iter().all(|sample| sample.language == "fr-gallo"));
    let samples = ok!(context.script("latn"));
    assert!(samples.iter().all(|sample| sample.script == "Latn"));
}

#[test]
fn simple_consistency() {
    let context = Context::default();
    let forward: usize = context.languages.values().map(Vec::len).sum();
    assert_eq!(forward, crate::DATA.len());
    assert!(forward > 700);
    // The samples of composites are listed several times.
    let backward: usize = context.scripts.values().map(Vec::len).sum();
    assert!(backward > forward);
}

#[test]
fn simple_order() {
    let context = Context::default();
    // A language might have several samples in a script, such as regional ones.
    for samples in context.languages.values() {
        assert!(
            samples
                .windows(2)
                .all(|pair| pair[0].script <= pair[1].script)
        );
    }
    for samples in context.scripts.values() {
        assert!(samples.windows(2).all(|pair| {
            (pair[0].language, pair[0].script) <= (pair[1].language, pair[1].script)
        }));
    }
}

#[test]
fn composite_languages() {
    let context = Context::default();
    let codes = |language: &str| {
        ok!(context.language(language))
            .iter()
            .map(|sample| sample.script)
            .collect::<Vec<_>>()
    };
    // A language is listed with the scripts its samples are given for.
    assert_eq!(codes("ja"), ["Hira", "Jpan", "Kana"]);
    assert_eq!(codes("ko"), ["Kore"]);
    assert_eq!(codes("zh"), ["Hans", "Hant", "Hebr"]);
}

#[test]
fn composite_scripts() {
    let context = Context::default();
    let find = |script: &str, language: &str| {
        ok!(context.script(script))
            .iter()
            .filter(|sample| sample.language == language)
            .map(|sample| (sample.script, ok!(sample.grid)))
            .collect::<Vec<_>>()
    };

    // A combination is listed as is and under each of its scripts.
    assert_eq!(find("Kore", "ko"), [("Kore", "은태")]);
    assert_eq!(find("Hang", "ko"), [("Kore", "은태")]);
    assert_eq!(find("Hani", "ko"), [("Kore", "은태")]);

    // A sample given specifically for a script is listed along with the combination.
    assert_eq!(find("Hani", "ja"), [("Jpan", "人間")]);
    assert_eq!(find("Hira", "ja"), [("Hira", "にんげん"), ("Jpan", "人間")]);
    assert_eq!(find("Kana", "ja"), [("Jpan", "人間"), ("Kana", "ニンゲン")]);

    // All variants are listed.
    assert_eq!(find("Hani", "zh"), [("Hans", "由在"), ("Hant", "由在")]);
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
