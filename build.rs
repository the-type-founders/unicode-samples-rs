use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(serde::Deserialize)]
struct Entry {
    language: String,
    script: String,
    population: Option<u32>,
    grid: Option<String>,
    line: Option<String>,
}

const DATA: &str = include_str!("vendor/lang/data.json");

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let root = ok!(std::env::var("OUT_DIR"));
    let path = Path::new(&root).join("data.rs");
    let mut file = ok!(File::create(&path));
    let data: Vec<Entry> = ok!(serde_json::from_str(DATA));
    let data = data
        .into_iter()
        .map(|entry| {
            format!(
                "({:?}, {:?}, {:?}, {:?}, {:?})",
                entry.language, entry.script, entry.population, entry.grid, entry.line,
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    ok!(write!(
        file,
        "const DATA: &[(&str, &str, Option<usize>, Option<&str>, Option<&str>)] = &[{data}];",
    ));
}
