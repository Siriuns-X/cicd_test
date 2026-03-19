#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),             // 半径
    Rectangle(f64, f64),     // 宽, 高
    Triangle(f64, f64, f64), // 三边
}

/// 计算面积（三角形用海伦公式）
pub fn area(shape: &Shape) -> f64 {
    // TODO: 补全 match
    match shape {
        Shape::Circle(x) => std::f64::consts::PI * x * x,
        Shape::Rectangle(x, y) => x * y,
        Shape::Triangle(x, y, z) => {
            let p = (x + y + z) / 2.0;
            (p * (p - x) * (p - y) * (p - z)).sqrt()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circle() {
        assert!((area(&Shape::Circle(1.0)) - std::f64::consts::PI).abs() < 1e-9);
    }
    #[test]
    fn test_rect() {
        assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    }
    #[test]
    fn test_tri() {
        assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-9);
    }
}
