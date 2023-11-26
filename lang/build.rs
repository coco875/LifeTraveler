use regex::Regex;
use std::fs;
use std::{collections::HashMap, io::Write, path};
use walkdir::WalkDir;

fn detect_lang(file_content: &str, lang: &mut HashMap<String, HashMap<String, String>>) {
    let re = Regex::new(r#"add_lang!\((\w+) *, *"([\w_]+)" *, *"([\w ]+)"\);"#).unwrap();
    for cap in re.captures_iter(file_content) {
        let lang_name = cap[1].to_string();
        let key = cap[2].to_string();
        let value = cap[3].to_string();
        if !lang.contains_key(&lang_name) {
            lang.insert(lang_name.clone(), HashMap::new());
        }
        lang.get_mut(&lang_name).unwrap().insert(key, value);
    }
}

fn detect_lang_file(
    file_content: &str,
    lang: &mut HashMap<String, HashMap<String, String>>,
    path: &path::Path,
) {
    let re = Regex::new(r#"add_lang_from_file!\((\w+) *, *"([\w_.]+)"\);"#).unwrap();
    for cap in re.captures_iter(file_content) {
        let lang_name = cap[1].to_string();
        let file_name = cap[2].to_string();
        let file_path = path.parent().unwrap().join(file_name);
        let file_content = fs::read_to_string(file_path).unwrap();
        let re = Regex::new(r####""*([\w_]+)"* *: *"*([\w ]+)"*"####).unwrap();
        for cap in re.captures_iter(&file_content) {
            let key = cap[1].to_string();
            let value = cap[2].to_string();
            if !lang.contains_key(&lang_name) {
                lang.insert(lang_name.clone(), HashMap::new());
            }
            lang.get_mut(&lang_name).unwrap().insert(key, value);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../core/");

    let mut lang: HashMap<String, HashMap<String, String>> = HashMap::new();

    for entry in WalkDir::new("..")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            !e.file_type().is_dir()
                && !e.path().to_str().unwrap().contains("target")
                && !e.path().to_str().unwrap().contains("git")
        })
    {
        let path = entry.path();
        if !path.to_str().unwrap().ends_with(".rs") {
            continue;
        }
        let file_content = fs::read_to_string(path).unwrap();
        detect_lang(&file_content, &mut lang);
        detect_lang_file(&file_content, &mut lang, path);
    }

    if !path::Path::new("../lang_output").exists() {
        fs::create_dir("../lang_output").unwrap();
    }

    for (lang_name, lang_content) in lang {
        let mut file = fs::File::create(format!("../lang_output/{}.json", lang_name)).unwrap();
        let mut content = String::new();
        for (key, value) in lang_content {
            content = content + &format!("\"{}\": \"{}\",\n", key, value);
        }
        content = content.trim_end_matches(",\n").to_string();
        content = format!("{{\n{}\n}}", content);
        file.write_all(content.as_bytes()).unwrap();
    }
}
