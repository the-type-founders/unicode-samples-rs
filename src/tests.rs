use crate::{COMPOSITES, Context, Language, Script};

#[test]
fn simple_languages() {
    let languages = Context::languages();
    let scripts = &languages["en"];
    assert!(scripts.contains(&Script {
        code: "latn",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(scripts.iter().any(|script| script.code == "brai"));
    assert!(languages.keys().all(|code| *code == code.to_lowercase()));
}

#[test]
fn simple_scripts() {
    let scripts = Context::scripts();
    let languages = &scripts["latn"];
    assert!(languages.contains(&Language {
        code: "en",
        population: Some(1636848718),
        grid: Some("Hh"),
        line: Some("Whereas disregard and contempt for human rights have resulted"),
    }));
    assert!(languages.iter().any(|language| language.code == "fr"));
    assert!(scripts.keys().all(|code| *code == code.to_lowercase()));
}

#[test]
fn simple_consistency() {
    let languages = Context::languages();
    let scripts = Context::scripts();
    let forward: usize = languages.values().map(Vec::len).sum();
    let backward: usize = scripts.values().map(Vec::len).sum();
    assert_eq!(forward, backward);
    assert!(forward > 700);
}

#[test]
fn composite_languages() {
    let languages = Context::languages();
    let codes = |language: &str| {
        languages[language]
            .iter()
            .map(|script| script.code)
            .collect::<Vec<_>>()
    };
    assert_eq!(codes("ko"), ["hang", "hani", "kore"]);
    assert_eq!(codes("zh"), ["hani", "hans", "hant", "hebr"]);
}

#[test]
fn composite_scripts() {
    let scripts = Context::scripts();
    let find = |script: &str, language: &str| {
        scripts[script]
            .iter()
            .filter(|candidate| candidate.code == language)
            .cloned()
            .collect::<Vec<_>>()
    };

    // A combination is listed as is and under each of its scripts.
    let korean = find("kore", "ko");
    assert_eq!(korean.len(), 1);
    assert_eq!(korean[0].grid, Some("은태"));
    assert_eq!(find("hang", "ko"), korean);
    assert_eq!(find("hani", "ko"), korean);

    // A sample given specifically for a script takes precedence.
    assert_eq!(find("hani", "ja"), find("jpan", "ja"));
    assert_eq!(find("hira", "ja").len(), 1);
    assert_eq!(find("hira", "ja")[0].grid, Some("にんげん"));
    assert_eq!(find("kana", "ja").len(), 1);
    assert_eq!(find("kana", "ja")[0].grid, Some("ニンゲン"));

    // The most populous variant wins.
    assert_eq!(find("hani", "zh"), find("hans", "zh"));
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
    let scripts = Context::scripts();
    assert!(codes.iter().all(|code| scripts.contains_key(code)));
}
