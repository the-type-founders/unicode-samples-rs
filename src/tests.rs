use crate::{Context, Language, Script};

#[test]
fn languages() {
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
fn scripts() {
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
fn consistency() {
    let languages = Context::languages();
    let scripts = Context::scripts();
    let forward: usize = languages.values().map(Vec::len).sum();
    let backward: usize = scripts.values().map(Vec::len).sum();
    assert_eq!(forward, backward);
    assert!(forward > 700);
}
