// src/lib.rs - полностью переписываем без глобального состояния
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned};
use quote::quote;

/// Основной макрос для пометки кода, который нужно преобразовать в TypeScript
#[proc_macro_attribute]
pub fn rust2ts(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    
    // Сразу генерируем TypeScript код для этого элемента
    let generated = generate_for_item(&input);
    
    // Сохраняем сгенерированный код в файл
    if !generated.is_empty() {
        // Используем out_dir для сохранения
        save_generated_code(&generated);
    }
    
    // Возвращаем оригинальный Rust код
    TokenStream::from(quote! { #input })
}

/// Генерирует TypeScript код для одного элемента
fn generate_for_item(item: &syn::Item) -> String {
    match item {
        syn::Item::Struct(item_struct) => {
            generate_struct(item_struct)
        }
        
        syn::Item::Fn(item_fn) => {
            generate_function(item_fn)
        }
        
        syn::Item::Const(item_const) => {
            generate_constant(item_const)
        }
        
        _ => String::new(), // Игнорируем другие элементы
    }
}

/// Генерация TypeScript интерфейса для структуры
fn generate_struct(item_struct: &syn::ItemStruct) -> String {
    let name = &item_struct.ident;
    
    let fields: Vec<String> = item_struct.fields.iter()
        .filter_map(|f| {
            f.ident.as_ref().map(|ident| {
                let type_ts = rust_type_to_ts(&f.ty);
                format!("    {}: {};", ident, type_ts)
            })
        })
        .collect();
    
    if fields.is_empty() {
        return String::new();
    }
    
    format!("export interface {} {{\n{}\n}}\n\n", name, fields.join("\n"))
}

/// Генерация TypeScript объявления функции
fn generate_function(item_fn: &syn::ItemFn) -> String {
    let name = &item_fn.sig.ident;
    
    // Параметры
    let params: Vec<String> = item_fn.sig.inputs.iter()
        .filter_map(|input| {
            if let syn::FnArg::Typed(pat_type) = input {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let type_ts = rust_type_to_ts(&pat_type.ty);
                    Some(format!("{}: {}", pat_ident.ident, type_ts))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    // Возвращаемый тип
    let return_type = match &item_fn.sig.output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, ty) => rust_type_to_ts(ty),
    };
    
    // Пока только объявление, без тела
    format!("export function {}({}): {};\n\n", 
        name, params.join(", "), return_type)
}

/// Генерация константы
fn generate_constant(item_const: &syn::ItemConst) -> String {
    let name = &item_const.ident;
    let type_ts = rust_type_to_ts(&item_const.ty);
    
    // Пока только объявление
    format!("export const {}: {};\n\n", name, type_ts)
}

/// Конвертация Rust типа в TypeScript тип
fn rust_type_to_ts(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(ident) = type_path.path.get_ident() {
                match ident.to_string().as_str() {
                    "i32" | "f64" | "i16" | "u32" | "usize" => "number".to_string(),
                    "bool" => "boolean".to_string(),
                    "String" => "string".to_string(),
                    name => name.to_string(), // Пользовательские типы
                }
            } else {
                "any".to_string()
            }
        }
        
        syn::Type::Array(type_array) => {
            let elem_type = rust_type_to_ts(&type_array.elem);
            format!("{}[]", elem_type)
        }
        
        syn::Type::Reference(type_ref) => {
            // Для &str тоже string
            if let syn::Type::Path(path) = &*type_ref.elem {
                if let Some(ident) = path.path.get_ident() {
                    if ident == "str" {
                        return "string".to_string();
                    }
                }
            }
            rust_type_to_ts(&type_ref.elem)
        }
        
        _ => "any".to_string(),
    }
}

/// Сохраняет сгенерированный код в файл
fn save_generated_code(code: &str) {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    
    // Получаем OUT_DIR из переменных окружения Cargo
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let rust2ts_dir = Path::new(&out_dir).join("rust2ts");
        
        // Создаем директорию, если её нет
        if let Ok(_) = fs::create_dir_all(&rust2ts_dir) {
            let ts_file = rust2ts_dir.join("generated.ts");
            
            // Открываем файл для добавления (append mode)
            if let Ok(mut file) = File::options()
                .create(true)
                .append(true)
                .open(ts_file) 
            {
                let _ = writeln!(file, "{}", code);
            }
        }
    }
}

/// Макрос для генерации TypeScript для всего модуля
#[proc_macro_attribute]
pub fn rust2ts_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    
    if let syn::Item::Mod(item_mod) = &input {
        if let Some((_, items)) = &item_mod.content {
            let mut all_generated = String::new();
            
            // Генерируем код для каждого элемента модуля
            for item in items {
                let generated = generate_for_item(item);
                all_generated.push_str(&generated);
            }
            
            // Сохраняем весь сгенерированный код
            if !all_generated.is_empty() {
                save_generated_code(&all_generated);
            }
        }
    }
    
    // Возвращаем оригинальный код
    TokenStream::from(quote! { #input })
}