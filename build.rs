// build.rs
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=examples");
    
    // Получаем OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let rust2ts_dir = Path::new(&out_dir).join("rust2ts");
    
    // Создаем директорию
    fs::create_dir_all(&rust2ts_dir).unwrap();
    
    // Создаем начальный файл с заголовком
    let ts_file = rust2ts_dir.join("generated.ts");
    if !ts_file.exists() {
        File::create(&ts_file).unwrap();
    }
    
    // Также создаем JS файл (пока пустой)
    let js_file = rust2ts_dir.join("generated.js");
    if !js_file.exists() {
        let mut file = File::create(&js_file).unwrap();
        writeln!(file, "// JavaScript implementations will be generated here").unwrap();
        writeln!(file, "// Currently only TypeScript types are generated").unwrap();
    }
    
    // Создаем файл с инструкциями
    let info_file = rust2ts_dir.join("INFO.txt");
    fs::write(info_file, 
        "rust2ts v0.1.0\n\
         TypeScript types are generated in generated.ts\n\
         JavaScript implementations will be added in future versions.\n").unwrap();
}