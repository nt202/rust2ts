// tests/integration.rs
use rust2ts::{rust2ts, rust2ts_module};

#[test]
fn test_struct_generation() {
    #[rust2ts]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[rust2ts]
    struct User {
        name: String,
        age: i32,
        active: bool,
    }
    
    // Пока просто проверяем, что компилируется
    let point = Point { x: 10, y: 20 };
    let user = User { 
        name: "John".to_string(), 
        age: 30, 
        active: true 
    };
    
    assert_eq!(point.x, 10);
    assert_eq!(user.name, "John");
}

#[test]
fn test_function_generation() {
    use rust2ts::{rust2ts, rust2ts_module};

    #[rust2ts]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    #[rust2ts]
    fn greet(name: String) -> String {
        format!("Hello, {}!", name)
    }
    
    assert_eq!(add(5, 10), 15);
    assert_eq!(greet("World".to_string()), "Hello, World!");
}

#[test]
fn test_module_generation() {
    use rust2ts::{rust2ts, rust2ts_module};

    #[rust2ts_module]
    mod math {
        use rust2ts::{rust2ts, rust2ts_module};

        #[rust2ts]
        pub const PI: f64 = 3.14159;
        
        #[rust2ts]
        pub fn circle_area(radius: f64) -> f64 {
            PI * radius * radius
        }
    }
    
    // Используем через полный путь, т.к. модуль
    use math::{PI, circle_area};
    
    assert!((PI - 3.14159).abs() < 0.00001);
    assert!((circle_area(2.0) - 12.56636).abs() < 0.00001);
}