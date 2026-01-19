// tests/simple_test.rs
use rust2ts::{rust2ts, rust2ts_module};

#[test]
fn test_basic_generation() {
    // Этот код должен скомпилироваться
    // И в OUT_DIR/rust2ts/generated.ts должны появиться интерфейсы
    
    #[rust2ts]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[rust2ts]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    #[rust2ts]
    const DEFAULT_VALUE: i32 = 42;
    
    // Проверяем, что Rust код работает
    let p = Point { x: 10, y: 20 };
    assert_eq!(add(5, 15), 20);
    assert_eq!(DEFAULT_VALUE, 42);
}

#[test]
fn test_module_generation() {
    use rust2ts::{rust2ts, rust2ts_module};

    #[rust2ts_module]
    mod math {
        use rust2ts::{rust2ts, rust2ts_module};

        #[rust2ts]
        pub struct Circle {
            pub radius: f64,
        }
        
        #[rust2ts]
        pub fn area(circle: Circle) -> f64 {
            std::f64::consts::PI * circle.radius * circle.radius
        }
    }
    
    // Используем код
    use math::{Circle, area};
    
    let circle = Circle { radius: 2.0 };
    let a = area(circle);
    assert!(a > 12.5 && a < 12.6);
}
