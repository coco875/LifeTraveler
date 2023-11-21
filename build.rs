use std::{path::Path, vec, io::Write};
use regex::Regex;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=core/");
    let current_dir = env::current_dir().unwrap_or_else(|_| panic!("Failed to get current dir"));

    let file_outpath = Path::new(&current_dir).join("register").join("src").join("blocks.rs");
    let mut file_block = fs::File::create(file_outpath).unwrap();
    let mut file_block_str = String::new();
    file_block_str.push_str("// This file is generated by build.rs\n");
    file_block_str.push_str("use block::Block;\nuse block::SimpleBlock;\n\n");

    let file_outpath = Path::new(&current_dir).join("register").join("src").join("items.rs");
    let mut file_item = fs::File::create(file_outpath).unwrap();
    let mut file_item_str = String::new();
    file_item_str.push_str("// This file is generated by build.rs\n");
    file_item_str.push_str("use item::Item;\nuse item::SimpleItem;\n\n");
    
    let mut block_register = vec![];
    let mut item_register = vec![];

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        let mut lib_path = path_str.replace("src\\", "");
        lib_path = lib_path.replace("src/", "");
        lib_path = lib_path.replace(".\\", "");
        lib_path = lib_path.replace("./", "");
        lib_path = lib_path.replace(".rs", "");
        lib_path = lib_path.replace("\\", "::");
        lib_path = lib_path.replace("/", "::");
        
        if path_str.contains("target") {
            continue;
        }

        if path_str.ends_with(".rs") {
            let file_name = fs::read_to_string(path).unwrap();
            let re = Regex::new(r"#\[register\(Block\)\]\npub mod (\w+)").unwrap();
            for cap in re.captures_iter(&file_name) {
                file_block_str.push_str(&format!("pub use {}::{};\n", lib_path, &cap[1]));
                block_register.push(cap[1].to_string());
            }

            let re = Regex::new(r"#\[register\(Item\)\]\npub mod (\w+)").unwrap();
            for cap in re.captures_iter(&file_name) {
                file_item_str.push_str(&format!("pub use {}::{};\n", lib_path, &cap[1]));
                item_register.push(cap[1].to_string());
            }
        }
    }

    file_block_str.push_str("\npub static REGISTER_BLOCK: &[fn(Block) -> SimpleBlock] = &[\n");

    let mut n = 0;

    for i in &block_register {
        file_block_str.push_str(&format!("    {}::load,{}// {}\n", i, "\t".repeat(4-((i.len()+2)/4)),n));
        n += 1;
    }

    file_block_str.push_str("];\n\n");

    n = 0;

    for i in &block_register {
        file_block_str.push_str(&format!("pub const {}_ID: u16 = {};\n", i.to_uppercase(), n));
        n += 1;
    }

    file_item_str.push_str("\npub static REGISTER_ITEM: &[fn(Item) -> SimpleItem] = &[\n");

    n = 0;

    for i in &item_register {
        file_item_str.push_str(&format!("    {}::load,{}// {}\n", i, "\t".repeat(4-((i.len())/4)),n));
        n += 1;
    }

    file_item_str.push_str("];\n\n");

    n = 0;

    for i in &item_register {
        file_item_str.push_str(&format!("pub const {}_ID: i32 = {};\n", i.to_uppercase(), n));
        n += 1;
    }

    file_block.write_all(file_block_str.as_bytes()).unwrap();
    file_item.write_all(file_item_str.as_bytes()).unwrap();

    file_block.flush().unwrap();
    file_item.flush().unwrap();

    // println!("cargo:rerun-if-changed=block/");
    // println!("cargo:rerun-if-changed=item/");
    // println!("cargo:rerun-if-changed=core/");
    // println!("cargo:rerun-if-changed=register/");
}
