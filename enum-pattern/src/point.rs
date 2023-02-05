struct Point {
    x: u32,
    y: u32,
}

#[test]
fn test_match() {
    let point = Point { x: 0, y: 1000 };

    let result = match point {
        Point { x: 0, y: height } => println!("straight up {} meters", height),
        Point { x, y } => println!("at ({}m, {}m)", x, y),
    };

    result
}
