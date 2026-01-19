// build.rs (в корне проекта)
fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=examples/chess/src/lib.rs");
    
    // В будущем здесь будет:
    // 1. Поиск всех #[rust2ts] анотаций
    // 2. Генерация TypeScript
    // 3. Запись в файлы
    
    #[cfg(feature = "generate")]
    {
        eprintln!("[rust2ts] Generation feature is enabled");
        // TODO: Реальная генерация
    }
}
