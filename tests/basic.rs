// tests/basic.rs
use rust2ts::{rust2ts, rust2ts_module};

#[test]
fn test_basic_struct() {
    #[rust2ts]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[rust2ts]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // Просто проверяем, что компилируется
    let point = Point { x: 10, y: 20 };
    let result = add(5, 10);
    assert_eq!(result, 15);
}
