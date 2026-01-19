// src/lib.rs
use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};
use quote::quote;

/// Основной макрос для пометки кода, который нужно преобразовать в TypeScript
#[proc_macro_attribute]
pub fn rust2ts(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    
    // Пока просто возвращаем оригинальный код
    // Но добавляем логи для отладки
    eprintln!("[rust2ts] Processing item: {:?}", input);
    
    TokenStream::from(quote! { #input })
}

/// Макрос для пометки всего модуля
#[proc_macro_attribute]
pub fn rust2ts_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    
    // TODO: Обработка всего модуля
    TokenStream::from(quote! { #input })
}
